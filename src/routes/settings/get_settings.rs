use actix_web::{HttpMessage, HttpRequest, HttpResponse, get, web};
use base64::Engine;
use diesel::prelude::*;
use serde::Serialize;

use crate::db::DbPool;
use crate::middleware::AuthUser;
use crate::schema::users;

#[derive(Serialize)]
struct SettingsResponse {
    email: String,
    otp_enabled: bool,
    profile_image: Option<String>,
}

#[get("")]
pub async fn get_settings(
    pool: web::Data<DbPool>,
    req: HttpRequest,
) -> Result<HttpResponse, actix_web::Error> {
    let user_id = req.extensions().get::<AuthUser>().unwrap().0;
    let pool_inner = pool.clone();

    let (email, otp_enabled, image_bytes, image_ct) = web::block(move || {
        let mut conn = pool_inner.get().expect("db pool exhausted");
        users::table
            .filter(users::id.eq(user_id))
            .select((
                users::email,
                users::otp_enabled,
                users::profile_image,
                users::profile_image_content_type,
            ))
            .first::<(String, bool, Option<Vec<u8>>, Option<String>)>(&mut conn)
    })
    .await
    .map_err(|_| actix_web::error::ErrorInternalServerError("Server error"))?
    .map_err(|_| actix_web::error::ErrorNotFound("User not found"))?;

    let profile_image = match (image_bytes, image_ct) {
        (Some(bytes), Some(ct)) => Some(format!(
            "data:{};base64,{}",
            ct,
            base64::engine::general_purpose::STANDARD.encode(&bytes)
        )),
        _ => None,
    };

    Ok(HttpResponse::Ok().json(SettingsResponse { email, otp_enabled, profile_image }))
}
