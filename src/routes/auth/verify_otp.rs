use actix_web::{HttpResponse, post, web};
use actix_web::cookie::{Cookie, SameSite, time::Duration};
use chrono::Utc;
use diesel::prelude::*;
use paseto_json::RegisteredClaims;
use paseto_v4::UnencryptedToken;
use serde::Deserialize;
use std::time::Duration as StdDuration;
use uuid::Uuid;

use crate::db::DbPool;
use crate::helpers::auth::paseto::local_key;
use crate::schema::{otp_codes, users};

#[derive(Deserialize)]
struct VerifyOtpRequest {
    email: String,
    code: String,
}

enum VerifyError {
    InvalidCode,
    NotFound,
    Db(diesel::result::Error),
}

impl From<diesel::result::Error> for VerifyError {
    fn from(e: diesel::result::Error) -> Self {
        VerifyError::Db(e)
    }
}

#[post("/verify-otp")]
pub async fn post_verify_otp(
    pool: web::Data<DbPool>,
    body: web::Json<VerifyOtpRequest>,
) -> Result<HttpResponse, actix_web::Error> {
    let email = body.email.clone();
    let code = body.code.clone();
    let pool_inner = pool.clone();

    let (user_id, is_admin) = web::block(move || -> Result<(Uuid, bool), VerifyError> {
        let mut conn = pool_inner.get().expect("db pool exhausted");
        let now = Utc::now();

        let (user_id, is_admin): (Uuid, bool) = users::table
            .filter(users::email.eq(&email))
            .select((users::id, users::is_admin))
            .first::<(Uuid, bool)>(&mut conn)
            .map_err(|e| match e {
                diesel::result::Error::NotFound => VerifyError::NotFound,
                other => VerifyError::Db(other),
            })?;

        let otp: Option<(Uuid, String)> = otp_codes::table
            .filter(
                otp_codes::user_id
                    .eq(user_id)
                    .and(otp_codes::used.eq(false))
                    .and(otp_codes::expires_at.gt(now)),
            )
            .select((otp_codes::id, otp_codes::code))
            .order(otp_codes::created_at.desc())
            .first::<(Uuid, String)>(&mut conn)
            .optional()?;

        let (otp_id, stored_code) = otp.ok_or(VerifyError::NotFound)?;

        if stored_code != code {
            return Err(VerifyError::InvalidCode);
        }

        diesel::update(otp_codes::table.filter(otp_codes::id.eq(otp_id)))
            .set(otp_codes::used.eq(true))
            .execute(&mut conn)?;

        Ok((user_id, is_admin))
    })
    .await
    .map_err(|_| actix_web::error::ErrorInternalServerError("Server error"))?
    .map_err(|e| match e {
        VerifyError::InvalidCode => actix_web::error::ErrorUnauthorized("Invalid code"),
        VerifyError::NotFound => actix_web::error::ErrorUnauthorized("Code expired or not found"),
        VerifyError::Db(_) => actix_web::error::ErrorInternalServerError("Database error"),
    })?;

    let key = local_key()?;
    let claims = RegisteredClaims::now(StdDuration::from_secs(24 * 3600))
        .for_subject(user_id.to_string());
    let token = UnencryptedToken::new(claims)
        .encrypt(&key)
        .map_err(|_| actix_web::error::ErrorInternalServerError("Token creation failed"))?
        .to_string();

    let session = Cookie::build("session", token)
        .http_only(true)
        .secure(true)
        .same_site(SameSite::Lax)
        .path("/")
        .max_age(Duration::hours(24))
        .finish();

    let admin_cookie = Cookie::build("is_admin", if is_admin { "true" } else { "false" })
        .http_only(false)
        .secure(true)
        .same_site(SameSite::Lax)
        .path("/")
        .max_age(Duration::hours(24))
        .finish();

    Ok(HttpResponse::Ok().cookie(session).cookie(admin_cookie).finish())
}
