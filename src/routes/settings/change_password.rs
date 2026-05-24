use actix_web::{HttpMessage, HttpRequest, HttpResponse, post, web};
use argon2::password_hash::SaltString;
use argon2::{Argon2, PasswordHash, PasswordHasher, PasswordVerifier};
use chrono::Utc;
use diesel::prelude::*;
use rand::rngs::OsRng;
use serde::Deserialize;
use uuid::Uuid;

use crate::db::DbPool;
use crate::middleware::AuthUser;
use crate::models::password_history::NewPasswordHistory;
use crate::schema::{password_history, users};

#[derive(Deserialize)]
struct ChangePasswordRequest {
    current_password: String,
    new_password: String,
    confirm_password: String,
}

enum ChangePasswordError {
    Db(diesel::result::Error),
    WrongCurrentPassword,
    PasswordReused,
    Hash,
    InvalidStoredHash,
}

impl From<diesel::result::Error> for ChangePasswordError {
    fn from(e: diesel::result::Error) -> Self {
        ChangePasswordError::Db(e)
    }
}

#[post("/change-password")]
pub async fn change_password(
    pool: web::Data<DbPool>,
    body: web::Json<ChangePasswordRequest>,
    req: HttpRequest,
) -> Result<HttpResponse, actix_web::Error> {
    let user_id = req.extensions().get::<AuthUser>().unwrap().0;

    if body.new_password != body.confirm_password {
        return Err(actix_web::error::ErrorBadRequest("Passwords do not match"));
    }

    let current_password = body.current_password.clone();
    let new_password = body.new_password.clone();

    web::block(move || -> Result<(), ChangePasswordError> {
        let mut conn = pool.get().expect("db pool exhausted");

        let current_hash: String = users::table
            .filter(users::id.eq(user_id))
            .select(users::password_hash)
            .first(&mut conn)?;

        let parsed = PasswordHash::new(&current_hash)
            .map_err(|_| ChangePasswordError::InvalidStoredHash)?;
        Argon2::default()
            .verify_password(current_password.as_bytes(), &parsed)
            .map_err(|_| ChangePasswordError::WrongCurrentPassword)?;

        // Fetch last 5 history entries (excludes current hash — checked separately above)
        let history: Vec<String> = password_history::table
            .filter(password_history::user_id.eq(user_id))
            .order(password_history::created_at.desc())
            .limit(4)
            .select(password_history::password_hash)
            .load(&mut conn)?;

        // Check new password against current hash + last 4 history entries = last 5 total
        for stored in std::iter::once(&current_hash).chain(history.iter()) {
            let parsed = PasswordHash::new(stored)
                .map_err(|_| ChangePasswordError::InvalidStoredHash)?;
            if Argon2::default()
                .verify_password(new_password.as_bytes(), &parsed)
                .is_ok()
            {
                return Err(ChangePasswordError::PasswordReused);
            }
        }

        let salt = SaltString::generate(&mut OsRng);
        let new_hash = Argon2::default()
            .hash_password(new_password.as_bytes(), &salt)
            .map_err(|_| ChangePasswordError::Hash)?
            .to_string();

        let now = Utc::now();

        // Archive current hash into history
        diesel::insert_into(password_history::table)
            .values(&NewPasswordHistory {
                id: Uuid::new_v4(),
                user_id,
                password_hash: current_hash,
                created_at: now,
            })
            .execute(&mut conn)?;

        // Prune history to 5 most recent entries
        let keep_ids: Vec<Uuid> = password_history::table
            .filter(password_history::user_id.eq(user_id))
            .order(password_history::created_at.desc())
            .limit(5)
            .select(password_history::id)
            .load(&mut conn)?;

        diesel::delete(
            password_history::table.filter(
                password_history::user_id
                    .eq(user_id)
                    .and(password_history::id.ne_all(keep_ids)),
            ),
        )
        .execute(&mut conn)?;

        diesel::update(users::table.filter(users::id.eq(user_id)))
            .set((
                users::password_hash.eq(new_hash),
                users::updated_at.eq(now),
            ))
            .execute(&mut conn)?;

        Ok(())
    })
    .await
    .map_err(|_| actix_web::error::ErrorInternalServerError("Server error"))?
    .map_err(|e| match e {
        ChangePasswordError::WrongCurrentPassword => {
            actix_web::error::ErrorUnauthorized("Current password is incorrect")
        }
        ChangePasswordError::PasswordReused => {
            actix_web::error::ErrorBadRequest("Password was used recently")
        }
        ChangePasswordError::Hash | ChangePasswordError::InvalidStoredHash => {
            actix_web::error::ErrorInternalServerError("Password hashing failed")
        }
        ChangePasswordError::Db(_) => {
            actix_web::error::ErrorInternalServerError("Database error")
        }
    })?;

    Ok(HttpResponse::Ok().finish())
}
