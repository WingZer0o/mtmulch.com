use actix_web::{HttpMessage, HttpRequest, HttpResponse, get, web};
use diesel::prelude::*;
use serde::Serialize;

use crate::db::DbPool;
use crate::middleware::AuthUser;
use crate::schema::users;

#[derive(Serialize)]
struct SettingsResponse {
    email: String,
    otp_enabled: bool,
}

#[get("/settings")]
pub async fn get_settings(
    pool: web::Data<DbPool>,
    req: HttpRequest,
) -> Result<HttpResponse, actix_web::Error> {
    let user_id = req.extensions().get::<AuthUser>().unwrap().0;
    let pool_inner = pool.clone();

    let (email, otp_enabled) = web::block(move || {
        let mut conn = pool_inner.get().expect("db pool exhausted");
        users::table
            .filter(users::id.eq(user_id))
            .select((users::email, users::otp_enabled))
            .first::<(String, bool)>(&mut conn)
    })
    .await
    .map_err(|_| actix_web::error::ErrorInternalServerError("Server error"))?
    .map_err(|_| actix_web::error::ErrorNotFound("User not found"))?;

    Ok(HttpResponse::Ok().json(SettingsResponse { email, otp_enabled }))
}
