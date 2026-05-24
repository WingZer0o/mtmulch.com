use actix_web::{HttpResponse, get, web};
use chrono::Utc;
use diesel::prelude::*;
use ed25519_dalek::{Signature, Verifier, VerifyingKey};
use serde::Deserialize;
use uuid::Uuid;

use crate::db::DbPool;
use crate::models::user_diesel::User;
use crate::schema::{failed_login_attempts, users};

const EXPIRY_SECS: i64 = 3600;

#[derive(Deserialize)]
struct UnlockQuery {
    uid: Uuid,
    ts: i64,
    sig: String,
}

#[get("/unlock-account")]
pub async fn get_unlock_account(
    pool: web::Data<DbPool>,
    query: web::Query<UnlockQuery>,
) -> Result<HttpResponse, actix_web::Error> {
    let uid = query.uid;
    let ts = query.ts;
    let sig_hex = query.sig.clone();

    let domain = std::env::var("APP_DOMAIN")
        .unwrap_or_else(|_| "http://localhost:3000".to_string());

    let now = Utc::now().timestamp();
    if ts > now || now - ts > EXPIRY_SECS {
        return Err(actix_web::error::ErrorBadRequest("Unlock link has expired"));
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

    // Already unlocked — redirect without error
    if !user.is_locked {
        return Ok(HttpResponse::SeeOther()
            .append_header(("Location", format!("{}/login?unlocked=1", domain)))
            .finish());
    }

    let pk_hex = user
        .unlock_public_key
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

    // Signature valid — unlock account, clear key, clear failed attempts
    web::block(move || {
        let mut conn = pool.get().expect("db pool exhausted");

        diesel::update(users::table.filter(users::id.eq(uid)))
            .set((
                users::is_locked.eq(false),
                users::unlock_public_key.eq::<Option<String>>(None),
            ))
            .execute(&mut conn)?;

        diesel::delete(
            failed_login_attempts::table.filter(failed_login_attempts::user_id.eq(uid)),
        )
        .execute(&mut conn)?;

        Ok::<(), diesel::result::Error>(())
    })
    .await
    .map_err(|_| actix_web::error::ErrorInternalServerError("Server error"))?
    .map_err(|_| actix_web::error::ErrorInternalServerError("Database error"))?;

    Ok(HttpResponse::SeeOther()
        .append_header(("Location", format!("{}/login?unlocked=1", domain)))
        .finish())
}
