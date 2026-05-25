use actix_web::{HttpResponse, post, web};
use chrono::Utc;
use diesel::prelude::*;
use ed25519_dalek::{Signer, SigningKey};
use rand::RngCore;
use rand::rngs::OsRng;
use serde::Deserialize;

use crate::db::DbPool;
use crate::email_queue::{EmailJob, EmailQueue};
use crate::models::email_list::EmailListEntry;
use crate::schema::email_list;

#[derive(Deserialize)]
pub struct UnsubscribeRequest {
    pub email: String,
}

#[post("/unsubscribe")]
pub async fn unsubscribe(
    pool: web::Data<DbPool>,
    queue: web::Data<EmailQueue>,
    body: web::Json<UnsubscribeRequest>,
) -> Result<HttpResponse, actix_web::Error> {
    let email = body.email.clone();

    let mut secret = [0u8; 32];
    OsRng.fill_bytes(&mut secret);
    let signing_key = SigningKey::from_bytes(&secret);
    let verifying_key = signing_key.verifying_key();
    let verifying_key_hex = hex::encode(verifying_key.to_bytes());

    let email_for_queue = email.clone();
    let vk_hex = verifying_key_hex.clone();
    let timestamp = Utc::now().timestamp();

    let result = web::block(move || -> Result<Option<EmailListEntry>, diesel::result::Error> {
        let mut conn = pool.get().expect("db pool exhausted");

        let existing: Option<EmailListEntry> = email_list::table
            .filter(email_list::email.eq(&email))
            .select(EmailListEntry::as_select())
            .first(&mut conn)
            .optional()?;

        match existing {
            Some(e) if e.verified => {
                diesel::update(email_list::table.filter(email_list::id.eq(e.id)))
                    .set(email_list::unsubscribe_public_key.eq(Some(&vk_hex)))
                    .execute(&mut conn)?;
                Ok(Some(e))
            }
            _ => Ok(None),
        }
    })
    .await
    .map_err(|_| actix_web::error::ErrorInternalServerError("Server error"))?
    .map_err(|_| actix_web::error::ErrorInternalServerError("Database error"))?;

    if let Some(entry) = result {
        let message = format!("{}:{}", entry.id, timestamp);
        let signature = signing_key.sign(message.as_bytes());
        let sig_hex = hex::encode(signature.to_bytes());

        let domain = std::env::var("APP_DOMAIN").unwrap_or_else(|_| "http://localhost:8080".to_string());
        let link = format!(
            "{}/api/blog/confirm-unsubscribe?uid={}&ts={}&sig={}",
            domain, entry.id, timestamp, sig_hex
        );

        queue
            .send(EmailJob::BlogUnsubscribe { to: email_for_queue, link })
            .map_err(|_| actix_web::error::ErrorInternalServerError("Email queue unavailable"))?;
    }

    Ok(HttpResponse::Ok().finish())
}
