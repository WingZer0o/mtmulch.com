use actix_web::{HttpMessage, HttpRequest, HttpResponse, post, web};
use chrono::Utc;
use diesel::prelude::*;
use serde::Deserialize;
use uuid::Uuid;

use crate::db::DbPool;
use crate::middleware::AuthUser;
use crate::schema::{otp_codes, users};

#[derive(Deserialize)]
struct VerifyRequest {
    code: String,
    enable: bool,
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

#[post("/settings/2fa/verify")]
pub async fn verify_otp(
    pool: web::Data<DbPool>,
    body: web::Json<VerifyRequest>,
    req: HttpRequest,
) -> Result<HttpResponse, actix_web::Error> {
    let user_id = req.extensions().get::<AuthUser>().unwrap().0;
    let pool_inner = pool.clone();
    let code = body.code.clone();
    let enable = body.enable;

    web::block(move || -> Result<(), VerifyError> {
        let mut conn = pool_inner.get().expect("db pool exhausted");
        let now = Utc::now();

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

        diesel::update(users::table.filter(users::id.eq(user_id)))
            .set(users::otp_enabled.eq(enable))
            .execute(&mut conn)?;

        Ok(())
    })
    .await
    .map_err(|_| actix_web::error::ErrorInternalServerError("Server error"))?
    .map_err(|e| match e {
        VerifyError::InvalidCode => actix_web::error::ErrorUnauthorized("Invalid code"),
        VerifyError::NotFound => actix_web::error::ErrorUnauthorized("Code expired or not found"),
        VerifyError::Db(_) => actix_web::error::ErrorInternalServerError("Database error"),
    })?;

    Ok(HttpResponse::Ok().finish())
}
