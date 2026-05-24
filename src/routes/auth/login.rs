use actix_web::{HttpResponse, post, web};
use actix_web::cookie::{Cookie, SameSite, time::Duration};
use argon2::{Argon2, PasswordHash, PasswordVerifier};
use chrono::Utc;
use diesel::prelude::*;
use ed25519_dalek::{Signer, SigningKey};
use paseto_json::RegisteredClaims;
use paseto_v4::UnencryptedToken;
use rand::RngCore;
use rand::Rng;
use rand::rngs::OsRng;
use serde::{Deserialize, Serialize};
use std::time::Duration as StdDuration;
use uuid::Uuid;

use crate::db::DbPool;
use crate::email_queue::{EmailJob, EmailQueue};
use crate::helpers::auth::paseto::local_key;
use crate::models::failed_login_attempt::NewFailedLoginAttempt;
use crate::models::otp_code::NewOtpCode;
use crate::models::user_diesel::User;
use crate::schema::{failed_login_attempts, otp_codes, users};

const MAX_FAILED_ATTEMPTS: i64 = 5;

#[derive(Deserialize)]
struct LoginRequest {
    email: String,
    password: String,
}

#[derive(Serialize)]
struct LoginResponse {
    otp_required: bool,
}

fn build_session_cookie(user_id: Uuid) -> Result<Cookie<'static>, actix_web::Error> {
    let key = local_key()?;
    let claims = RegisteredClaims::now(StdDuration::from_secs(24 * 3600))
        .for_subject(user_id.to_string());
    let token = UnencryptedToken::new(claims)
        .encrypt(&key)
        .map_err(|_| actix_web::error::ErrorInternalServerError("Token creation failed"))?
        .to_string();

    Ok(Cookie::build("session", token)
        .http_only(false)
        .secure(true)
        .same_site(SameSite::None)
        .path("/")
        .max_age(Duration::hours(24))
        .finish())
}

fn build_is_admin_cookie(is_admin: bool) -> Cookie<'static> {
    Cookie::build("is_admin", if is_admin { "true" } else { "false" })
        .http_only(false)
        .secure(true)
        .same_site(SameSite::None)
        .path("/")
        .max_age(Duration::hours(24))
        .finish()
}

#[post("/login")]
pub async fn post_login(
    pool: web::Data<DbPool>,
    queue: web::Data<EmailQueue>,
    body: web::Json<LoginRequest>,
) -> Result<HttpResponse, actix_web::Error> {
    let email = body.email.clone();
    let password = body.password.clone();

    // --- 1. Fetch user ---
    let pool_inner = pool.clone();
    let user: User = web::block(move || {
        let mut conn = pool_inner.get().expect("db pool exhausted");
        users::table
            .filter(users::email.eq(&email))
            .select(User::as_select())
            .first::<User>(&mut conn)
    })
    .await
    .map_err(|_| actix_web::error::ErrorInternalServerError("Server error"))?
    .map_err(|e| match e {
        diesel::result::Error::NotFound => actix_web::error::ErrorUnauthorized("Invalid credentials"),
        _ => actix_web::error::ErrorInternalServerError("Database error"),
    })?;

    // --- 2. Check lockout before any password work ---
    if user.is_locked {
        return Err(actix_web::error::ErrorForbidden(
            "Account locked. Check your email for an unlock link.",
        ));
    }

    // --- 3. Verify password ---
    let hash = PasswordHash::new(&user.password_hash)
        .map_err(|_| actix_web::error::ErrorInternalServerError("Invalid password hash"))?;

    if Argon2::default().verify_password(password.as_bytes(), &hash).is_err() {
        // Pre-generate unlock keypair (only stored if this is the 5th failure)
        let mut secret = [0u8; 32];
        OsRng.fill_bytes(&mut secret);
        let signing_key = SigningKey::from_bytes(&secret);
        let verifying_key_hex = hex::encode(signing_key.verifying_key().to_bytes());
        let ts = Utc::now().timestamp();
        let user_id = user.id;
        let message = format!("{}:{}", user_id, ts);
        let sig_hex = hex::encode(signing_key.sign(message.as_bytes()).to_bytes());

        let pool_inner = pool.clone();
        let locked = web::block(move || {
            let mut conn = pool_inner.get().expect("db pool exhausted");

            diesel::insert_into(failed_login_attempts::table)
                .values(&NewFailedLoginAttempt {
                    id: Uuid::new_v4(),
                    user_id,
                    attempted_at: Utc::now(),
                })
                .execute(&mut conn)?;

            let count: i64 = failed_login_attempts::table
                .filter(failed_login_attempts::user_id.eq(user_id))
                .count()
                .get_result(&mut conn)?;

            if count >= MAX_FAILED_ATTEMPTS {
                diesel::update(users::table.filter(users::id.eq(user_id)))
                    .set((
                        users::is_locked.eq(true),
                        users::unlock_public_key.eq(Some(verifying_key_hex)),
                    ))
                    .execute(&mut conn)?;
                return Ok::<bool, diesel::result::Error>(true);
            }

            Ok(false)
        })
        .await
        .map_err(|_| actix_web::error::ErrorInternalServerError("Server error"))?
        .map_err(|_| actix_web::error::ErrorInternalServerError("Database error"))?;

        if locked {
            let domain = std::env::var("APP_DOMAIN")
                .unwrap_or_else(|_| "http://localhost:8080".to_string());
            let link = format!(
                "{}/api/auth/unlock-account?uid={}&ts={}&sig={}",
                domain, user_id, ts, sig_hex
            );
            queue
                .send(EmailJob::AccountLocked { to: user.email, link })
                .map_err(|_| actix_web::error::ErrorInternalServerError("Email queue unavailable"))?;
        }

        return Err(actix_web::error::ErrorUnauthorized("Invalid credentials"));
    }

    // --- 4. Password correct — clear failed attempts ---
    let user_id = user.id;
    let pool_inner = pool.clone();
    web::block(move || {
        let mut conn = pool_inner.get().expect("db pool exhausted");
        diesel::delete(
            failed_login_attempts::table.filter(failed_login_attempts::user_id.eq(user_id)),
        )
        .execute(&mut conn)
    })
    .await
    .map_err(|_| actix_web::error::ErrorInternalServerError("Server error"))?
    .map_err(|_| actix_web::error::ErrorInternalServerError("Database error"))?;

    // --- 5. Require email verification ---
    if !user.is_verified {
        return Err(actix_web::error::ErrorForbidden("Email not verified"));
    }

    // --- 6. No 2FA: issue session immediately ---
    if !user.otp_enabled {
        let session = build_session_cookie(user.id)?;
        let is_admin = build_is_admin_cookie(user.is_admin);
        return Ok(HttpResponse::Ok()
            .cookie(session)
            .cookie(is_admin)
            .json(LoginResponse { otp_required: false }));
    }

    // --- 7. 2FA path: generate OTP, persist, queue email ---
    let user_email = user.email.clone();
    let pool_inner = pool.clone();

    let code = web::block(move || {
        let mut conn = pool_inner.get().expect("db pool exhausted");
        let now = Utc::now();

        diesel::delete(
            otp_codes::table.filter(
                otp_codes::user_id.eq(user_id).and(otp_codes::expires_at.lt(now)),
            ),
        )
        .execute(&mut conn)?;

        let code = rand::thread_rng().gen_range(100_000u32..=999_999u32).to_string();

        diesel::insert_into(otp_codes::table)
            .values(&NewOtpCode {
                id: Uuid::new_v4(),
                user_id,
                code: code.clone(),
                expires_at: now + chrono::Duration::minutes(10),
                created_at: now,
            })
            .execute(&mut conn)?;

        Ok::<String, diesel::result::Error>(code)
    })
    .await
    .map_err(|_| actix_web::error::ErrorInternalServerError("Server error"))?
    .map_err(|_| actix_web::error::ErrorInternalServerError("Database error"))?;

    queue
        .send(EmailJob::Otp { to: user_email, code })
        .map_err(|_| actix_web::error::ErrorInternalServerError("Email queue unavailable"))?;

    Ok(HttpResponse::Accepted().json(LoginResponse { otp_required: true }))
}

