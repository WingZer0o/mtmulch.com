use actix_web::{HttpResponse, get, web};
use chrono::Utc;
use diesel::prelude::*;
use ed25519_dalek::{Verifier, VerifyingKey, Signature};
use serde::Deserialize;
use uuid::Uuid;

use crate::db::DbPool;
use crate::models::user_diesel::User;
use crate::schema::users;

const EXPIRY_SECS: i64 = 24 * 3600;

#[derive(Deserialize)]
struct VerifyQuery {
    uid: Uuid,
    ts: i64,
    sig: String,
}

#[get("/verify-email")]
pub async fn get_verify_email(
    pool: web::Data<DbPool>,
    query: web::Query<VerifyQuery>,
) -> Result<HttpResponse, actix_web::Error> {
    let uid = query.uid;
    let ts = query.ts;
    let sig_hex = query.sig.clone();

    let now = Utc::now().timestamp();
    if ts > now || now - ts > EXPIRY_SECS {
        return Err(actix_web::error::ErrorBadRequest("Link has expired"));
    }

    let sig_bytes: [u8; 64] = hex::decode(&sig_hex)
        .map_err(|_| actix_web::error::ErrorBadRequest("Invalid token"))?
        .try_into()
        .map_err(|_| actix_web::error::ErrorBadRequest("Invalid token"))?;

    let pool_read = pool.clone();
    let user: User = web::block(move || {
        let mut conn = pool_read.get().expect("db pool exhausted");
        users::table
            .filter(users::id.eq(uid))
            .select(User::as_select())
            .first::<User>(&mut conn)
    })
    .await
    .map_err(|_| actix_web::error::ErrorInternalServerError("Server error"))?
    .map_err(|e| match e {
        diesel::result::Error::NotFound => actix_web::error::ErrorBadRequest("Invalid token"),
        _ => actix_web::error::ErrorInternalServerError("Database error"),
    })?;

    let domain = std::env::var("APP_DOMAIN")
        .unwrap_or_else(|_| "http://localhost:3000".to_string());

    if user.is_verified {
        return Ok(HttpResponse::SeeOther()
            .append_header(("Location", format!("{}/login?verified=1", domain)))
            .finish());
    }

    let pk_hex = user
        .verification_public_key
        .ok_or_else(|| actix_web::error::ErrorBadRequest("Link already used"))?;

    let pk_bytes: [u8; 32] = hex::decode(&pk_hex)
        .map_err(|_| actix_web::error::ErrorInternalServerError("Invalid stored key"))?
        .try_into()
        .map_err(|_| actix_web::error::ErrorInternalServerError("Invalid stored key"))?;

    let verifying_key = VerifyingKey::from_bytes(&pk_bytes)
        .map_err(|_| actix_web::error::ErrorInternalServerError("Invalid stored key"))?;

    let signature = Signature::from_bytes(&sig_bytes);
    let message = format!("{}:{}", uid, ts);

    verifying_key
        .verify(message.as_bytes(), &signature)
        .map_err(|_| actix_web::error::ErrorBadRequest("Invalid token"))?;

    // Signature valid — mark user verified and clear the one-time key
    web::block(move || {
        let mut conn = pool.get().expect("db pool exhausted");
        diesel::update(users::table.filter(users::id.eq(uid)))
            .set((
                users::is_verified.eq(true),
                users::verification_public_key.eq::<Option<String>>(None),
            ))
            .execute(&mut conn)
    })
    .await
    .map_err(|_| actix_web::error::ErrorInternalServerError("Server error"))?
    .map_err(|_| actix_web::error::ErrorInternalServerError("Database error"))?;

    Ok(HttpResponse::SeeOther()
        .append_header(("Location", format!("{}/login?verified=1", domain)))
        .finish())
}
