use actix_web::{HttpResponse, post, web};
use argon2::password_hash::SaltString;
use rand::RngCore;
use rand::rngs::OsRng;
use argon2::{Argon2, PasswordHasher};
use chrono::Utc;
use diesel::prelude::*;
use ed25519_dalek::{Signer, SigningKey};
use serde::Deserialize;
use uuid::Uuid;

use crate::db::DbPool;
use crate::email_queue::{EmailJob, EmailQueue};
use crate::models::user_diesel::NewUser;
use crate::schema::users;

#[derive(Deserialize)]
struct RegisterRequest {
    email: String,
    password: String,
}

enum RegisterError {
    EmailTaken,
    Db(diesel::result::Error),
    Hash,
}

impl From<diesel::result::Error> for RegisterError {
    fn from(e: diesel::result::Error) -> Self {
        RegisterError::Db(e)
    }
}

#[post("/register")]
pub async fn post_register(
    pool: web::Data<DbPool>,
    queue: web::Data<EmailQueue>,
    body: web::Json<RegisterRequest>,
) -> Result<HttpResponse, actix_web::Error> {
    let email = body.email.clone();
    let password = body.password.clone();
    let pool_inner = pool.clone();

    // Generate ed25519 keypair and sign the verification message outside the
    // blocking thread — crypto is fast and this keeps the closure simpler.
    let mut secret = [0u8; 32];
    OsRng.fill_bytes(&mut secret);
    let signing_key = SigningKey::from_bytes(&secret);
    let verifying_key = signing_key.verifying_key();
    let verifying_key_hex = hex::encode(verifying_key.to_bytes());

    let user_id = Uuid::new_v4();
    let timestamp = Utc::now().timestamp();
    let message = format!("{}:{}", user_id, timestamp);
    let signature = signing_key.sign(message.as_bytes());
    let sig_hex = hex::encode(signature.to_bytes());

    let email_for_link = email.clone();
    let verifying_key_hex_for_insert = verifying_key_hex.clone();

    web::block(move || -> Result<(), RegisterError> {
        let mut conn = pool_inner.get().expect("db pool exhausted");

        let count: i64 = users::table
            .filter(users::email.eq(&email))
            .count()
            .get_result(&mut conn)?;

        if count > 0 {
            return Err(RegisterError::EmailTaken);
        }

        let salt = SaltString::generate(&mut OsRng);
        let password_hash = Argon2::default()
            .hash_password(password.as_bytes(), &salt)
            .map_err(|_| RegisterError::Hash)?
            .to_string();

        let now = Utc::now();
        let new_user = NewUser {
            id: user_id,
            email,
            password_hash,
            created_at: now,
            updated_at: now,
            verification_public_key: verifying_key_hex_for_insert,
        };

        diesel::insert_into(users::table)
            .values(&new_user)
            .execute(&mut conn)?;

        Ok(())
    })
    .await
    .map_err(|_| actix_web::error::ErrorInternalServerError("Server error"))?
    .map_err(|e| match e {
        RegisterError::EmailTaken => actix_web::error::ErrorConflict("Email already registered"),
        RegisterError::Hash => actix_web::error::ErrorInternalServerError("Password hashing failed"),
        RegisterError::Db(_) => actix_web::error::ErrorInternalServerError("Database error"),
    })?;

    let domain = std::env::var("APP_DOMAIN").unwrap_or_else(|_| "http://localhost:8080".to_string());
    let link = format!(
        "{}/api/auth/verify-email?uid={}&ts={}&sig={}",
        domain, user_id, timestamp, sig_hex
    );

    queue
        .send(EmailJob::Verification { to: email_for_link, link })
        .map_err(|_| actix_web::error::ErrorInternalServerError("Email queue unavailable"))?;

    Ok(HttpResponse::Created().finish())
}
