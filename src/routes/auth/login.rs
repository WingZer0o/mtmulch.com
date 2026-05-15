use actix_web::{HttpResponse, post, web};
use actix_web::cookie::{Cookie, SameSite, time::Duration};
use argon2::{Argon2, PasswordHash, PasswordVerifier};
use diesel::prelude::*;
use paseto_json::RegisteredClaims;
use paseto_v4::UnencryptedToken;
use serde::Deserialize;
use std::time::Duration as StdDuration;

use crate::db::DbPool;
use crate::models::user_diesel::User;
use crate::paseto::local_key;
use crate::schema::users;

#[derive(Deserialize)]
struct LoginRequest {
    email: String,
    password: String,
}

#[post("/login")]
pub async fn post_login(
    pool: web::Data<DbPool>,
    body: web::Json<LoginRequest>,
) -> Result<HttpResponse, actix_web::Error> {
    let email = body.email.clone();
    let pool_inner = pool.clone();

    let user: User = web::block(move || {
        let mut conn = pool_inner.get().expect("db pool exhausted");
        users::table
            .filter(users::email.eq(&email))
            .select(User::as_select())
            .first::<User>(&mut conn)
    })
    .await
    .map_err(|_| actix_web::error::ErrorInternalServerError("Server error"))?
    .map_err(|e| match e {
        diesel::result::Error::NotFound => actix_web::error::ErrorUnauthorized("Invalid credentials"),
        _ => actix_web::error::ErrorInternalServerError("Database error"),
    })?;

    let hash = PasswordHash::new(&user.password_hash)
        .map_err(|_| actix_web::error::ErrorInternalServerError("Invalid password hash"))?;
    Argon2::default()
        .verify_password(body.password.as_bytes(), &hash)
        .map_err(|_| actix_web::error::ErrorUnauthorized("Invalid credentials"))?;

    let key = local_key()?;
    let claims = RegisteredClaims::now(StdDuration::from_secs(24 * 3600))
        .for_subject(user.id.to_string());
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

    Ok(HttpResponse::Ok().cookie(session).finish())
}
