use actix_web::{HttpResponse, post, web};
use actix_web::cookie::{Cookie, SameSite, time::Duration};
use argon2::{Argon2, PasswordHash, PasswordVerifier};
use chrono::Utc;
use diesel::prelude::*;
use paseto_json::RegisteredClaims;
use paseto_v4::UnencryptedToken;
use rand::Rng;
use serde::{Deserialize, Serialize};
use std::time::Duration as StdDuration;
use uuid::Uuid;

use crate::db::DbPool;
use crate::email_queue::{EmailJob, EmailQueue};
use crate::helpers::auth::paseto::local_key;
use crate::models::otp_code::NewOtpCode;
use crate::models::user_diesel::User;
use crate::schema::{otp_codes, users};

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
        .http_only(true)
        .secure(true)
        .same_site(SameSite::Lax)
        .path("/")
        .max_age(Duration::hours(24))
        .finish())
}

#[post("/login")]
pub async fn post_login(
    pool: web::Data<DbPool>,
    queue: web::Data<EmailQueue>,
    body: web::Json<LoginRequest>,
) -> Result<HttpResponse, actix_web::Error> {
    let email = body.email.clone();
    let password = body.password.clone();
    let pool_inner = pool.clone();

    let user: User = web::block(move || {
        let mut conn = pool_inner.get().expect("db pool exhausted");
        users::table
            .filter(users::email.eq(&email))
            .select(User::as_select())
            .first::<User>(&mut conn)
    })
    .await
    .map_err(|e| {
        eprintln!("[login/select] blocking error: {e:?}");
        actix_web::error::ErrorInternalServerError("Server error")
    })?
    .map_err(|e| {
        eprintln!("[login/select] database error: {e:?}");
        match e {
            diesel::result::Error::NotFound => actix_web::error::ErrorUnauthorized("Invalid credentials"),
            _ => actix_web::error::ErrorInternalServerError("Database error"),
        }
    })?;

    let hash = PasswordHash::new(&user.password_hash)
        .map_err(|_| actix_web::error::ErrorInternalServerError("Invalid password hash"))?;
    Argon2::default()
        .verify_password(password.as_bytes(), &hash)
        .map_err(|_| actix_web::error::ErrorUnauthorized("Invalid credentials"))?;

    if !user.otp_enabled {
        let cookie = build_session_cookie(user.id)?;
        return Ok(HttpResponse::Ok()
            .cookie(cookie)
            .json(LoginResponse { otp_required: false }));
    }

    // 2FA path: generate OTP, persist, queue email
    let user_id = user.id;
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
    .map_err(|e| {
        eprintln!("[login/otp] blocking error: {e:?}");
        actix_web::error::ErrorInternalServerError("Server error")
    })?
    .map_err(|e| {
        eprintln!("[login/otp] database error: {e:?}");
        actix_web::error::ErrorInternalServerError("Database error")
    })?;

    queue
        .send(EmailJob { to: user_email, code })
        .map_err(|_| actix_web::error::ErrorInternalServerError("Email queue unavailable"))?;

    Ok(HttpResponse::Accepted().json(LoginResponse { otp_required: true }))
}
