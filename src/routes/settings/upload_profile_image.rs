use actix_multipart::Multipart;
use actix_web::{HttpMessage, HttpRequest, HttpResponse, post, web};
use base64::Engine;
use diesel::prelude::*;
use futures_util::StreamExt;
use serde::Serialize;

use crate::db::DbPool;
use crate::middleware::AuthUser;
use crate::schema::users;

const MAX_SIZE: usize = 5 * 1024 * 1024;
const ALLOWED_TYPES: &[&str] = &["image/jpeg", "image/png", "image/webp", "image/gif"];

#[derive(Serialize)]
struct UploadResponse {
    profile_image: String,
}

#[post("/profile-image")]
pub async fn upload_profile_image(
    req: HttpRequest,
    pool: web::Data<DbPool>,
    mut payload: Multipart,
) -> Result<HttpResponse, actix_web::Error> {
    let user_id = req.extensions().get::<AuthUser>().unwrap().0;

    while let Some(item) = payload.next().await {
        let mut field = item.map_err(|_| actix_web::error::ErrorBadRequest("Multipart error"))?;

        let content_type = field
            .content_type()
            .map(|ct| ct.to_string())
            .unwrap_or_default();

        if !ALLOWED_TYPES.contains(&content_type.as_str()) {
            return Ok(HttpResponse::BadRequest().body("Invalid file type. Use JPEG, PNG, WebP, or GIF."));
        }

        let mut bytes: Vec<u8> = Vec::new();
        while let Some(chunk) = field.next().await {
            let chunk = chunk.map_err(|_| actix_web::error::ErrorBadRequest("Upload error"))?;
            bytes.extend_from_slice(&chunk);
            if bytes.len() > MAX_SIZE {
                return Ok(HttpResponse::BadRequest().body("File exceeds 5 MB limit."));
            }
        }

        let content_type_clone = content_type.clone();
        let bytes_clone = bytes.clone();
        let pool_inner = pool.clone();

        web::block(move || {
            let mut conn = pool_inner.get().expect("db pool exhausted");
            diesel::update(users::table.filter(users::id.eq(user_id)))
                .set((
                    users::profile_image.eq(&bytes_clone),
                    users::profile_image_content_type.eq(&content_type_clone),
                    users::updated_at.eq(chrono::Utc::now()),
                ))
                .execute(&mut conn)
        })
        .await
        .map_err(|_| actix_web::error::ErrorInternalServerError("Server error"))?
        .map_err(|_| actix_web::error::ErrorInternalServerError("Database error"))?;

        let data_url = format!(
            "data:{};base64,{}",
            content_type,
            base64::engine::general_purpose::STANDARD.encode(&bytes)
        );

        return Ok(HttpResponse::Ok().json(UploadResponse { profile_image: data_url }));
    }

    Ok(HttpResponse::BadRequest().body("No image field found in upload."))
}
