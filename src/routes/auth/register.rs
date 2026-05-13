use actix_web::{HttpResponse, post, web};
use argon2::password_hash::SaltString;
use rand::rngs::OsRng;
use argon2::{Argon2, PasswordHasher};
use chrono::Utc;
use diesel::prelude::*;
use serde::Deserialize;
use uuid::Uuid;

use crate::db::DbPool;
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
    body: web::Json<RegisterRequest>,
) -> Result<HttpResponse, actix_web::Error> {
    let email = body.email.clone();
    let password = body.password.clone();
    let pool_inner = pool.clone();

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
            id: Uuid::new_v4(),
            email,
            password_hash,
            created_at: now,
            updated_at: now,
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

    Ok(HttpResponse::Created().finish())
}
