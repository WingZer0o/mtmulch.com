use actix_web::{HttpResponse, get, web};
use chrono::Utc;
use diesel::prelude::*;
use ed25519_dalek::{Signature, Verifier, VerifyingKey};
use serde::Deserialize;
use uuid::Uuid;

use crate::db::DbPool;
use crate::models::email_list::EmailListEntry;
use crate::schema::email_list;

const EXPIRY_SECS: i64 = 3600;

#[derive(Deserialize)]
struct VerifyQuery {
    uid: Uuid,
    ts: i64,
    sig: String,
}

#[get("/verify-subscription")]
pub async fn verify_subscription(
    pool: web::Data<DbPool>,
    query: web::Query<VerifyQuery>,
) -> Result<HttpResponse, actix_web::Error> {
    let uid = query.uid;
    let ts = query.ts;
    let sig_hex = query.sig.clone();

    let domain = std::env::var("APP_DOMAIN").unwrap_or_else(|_| "http://localhost:3000".to_string());

    let now = Utc::now().timestamp();
    if ts > now || now - ts > EXPIRY_SECS {
        return Err(actix_web::error::ErrorBadRequest("Link has expired"));
    }

    let sig_bytes: [u8; 64] = hex::decode(&sig_hex)
        .map_err(|_| actix_web::error::ErrorBadRequest("Invalid token"))?
        .try_into()
        .map_err(|_| actix_web::error::ErrorBadRequest("Invalid token"))?;

    let pool_read = pool.clone();
    let entry: EmailListEntry = web::block(move || {
        let mut conn = pool_read.get().expect("db pool exhausted");
        email_list::table
            .filter(email_list::id.eq(uid))
            .select(EmailListEntry::as_select())
            .first(&mut conn)
    })
    .await
    .map_err(|_| actix_web::error::ErrorInternalServerError("Server error"))?
    .map_err(|e| match e {
        diesel::result::Error::NotFound => actix_web::error::ErrorBadRequest("Invalid token"),
        _ => actix_web::error::ErrorInternalServerError("Database error"),
    })?;

    if entry.verified {
        return Ok(HttpResponse::SeeOther()
            .append_header(("Location", format!("{}/blog?subscribed=1", domain)))
            .finish());
    }

    let pk_hex = entry
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

    web::block(move || {
        let mut conn = pool.get().expect("db pool exhausted");
        diesel::update(email_list::table.filter(email_list::id.eq(uid)))
            .set((
                email_list::verified.eq(true),
                email_list::verification_public_key.eq::<Option<String>>(None),
            ))
            .execute(&mut conn)
    })
    .await
    .map_err(|_| actix_web::error::ErrorInternalServerError("Server error"))?
    .map_err(|_| actix_web::error::ErrorInternalServerError("Database error"))?;

    Ok(HttpResponse::SeeOther()
        .append_header(("Location", format!("{}/blog?subscribed=1", domain)))
        .finish())
}
