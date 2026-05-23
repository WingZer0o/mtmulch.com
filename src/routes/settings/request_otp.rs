use actix_web::{HttpMessage, HttpRequest, HttpResponse, post, web};
use chrono::Utc;
use diesel::prelude::*;
use rand::Rng;
use uuid::Uuid;

use crate::db::DbPool;
use crate::email_queue::{EmailJob, EmailQueue};
use crate::middleware::AuthUser;
use crate::models::otp_code::NewOtpCode;
use crate::schema::{otp_codes, users};

#[post("/settings/2fa/request")]
pub async fn request_otp(
    pool: web::Data<DbPool>,
    queue: web::Data<EmailQueue>,
    req: HttpRequest,
) -> Result<HttpResponse, actix_web::Error> {
    let user_id = req.extensions().get::<AuthUser>().unwrap().0;
    let pool_inner = pool.clone();

    let (user_email, code) = web::block(move || {
        let mut conn = pool_inner.get().expect("db pool exhausted");
        let now = Utc::now();

        // Clean up expired OTPs for this user
        diesel::delete(
            otp_codes::table.filter(
                otp_codes::user_id.eq(user_id).and(otp_codes::expires_at.lt(now)),
            ),
        )
        .execute(&mut conn)?;

        let code = rand::thread_rng().gen_range(100_000u32..=999_999u32).to_string();
        let expires_at = now + chrono::Duration::minutes(10);

        diesel::insert_into(otp_codes::table)
            .values(&NewOtpCode {
                id: Uuid::new_v4(),
                user_id,
                code: code.clone(),
                expires_at,
                created_at: now,
            })
            .execute(&mut conn)?;

        let email = users::table
            .filter(users::id.eq(user_id))
            .select(users::email)
            .first::<String>(&mut conn)?;

        Ok::<(String, String), diesel::result::Error>((email, code))
    })
    .await
    .map_err(|_| actix_web::error::ErrorInternalServerError("Server error"))?
    .map_err(|_| actix_web::error::ErrorInternalServerError("Database error"))?;

    queue
        .send(EmailJob { to: user_email, code })
        .map_err(|_| actix_web::error::ErrorInternalServerError("Email queue unavailable"))?;

    Ok(HttpResponse::Ok().finish())
}
