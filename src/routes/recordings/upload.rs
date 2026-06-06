use std::time::{SystemTime, UNIX_EPOCH};

use actix_web::{HttpMessage, HttpRequest, HttpResponse, post, web};
use diesel::prelude::*;
use futures_util::StreamExt;
use uuid::Uuid;

use crate::db::DbPool;
use crate::middleware::AuthUser;
use crate::models::recording::NewRecording;
use crate::schema::recordings;

#[post("/upload")]
pub async fn upload(
    pool: web::Data<DbPool>,
    req: HttpRequest,
    mut payload: web::Payload,
) -> actix_web::Result<HttpResponse> {
    let user_id = req.extensions().get::<AuthUser>().unwrap().0;

    let filename = req
        .headers()
        .get("X-Filename")
        .and_then(|v| v.to_str().ok())
        .map(sanitize_filename)
        .unwrap_or_else(default_filename);

    let mut data: Vec<u8> = Vec::new();
    while let Some(chunk) = payload.next().await {
        let chunk = chunk.map_err(actix_web::error::ErrorInternalServerError)?;
        data.extend_from_slice(&chunk);
    }

    let size_bytes = data.len() as i64;
    let new_rec = NewRecording {
        id: Uuid::new_v4(),
        user_id,
        filename: filename.clone(),
        size_bytes,
        data,
    };

    web::block(move || {
        let mut conn = pool.get().expect("db pool exhausted");
        diesel::insert_into(recordings::table)
            .values(&new_rec)
            .execute(&mut conn)
    })
    .await
    .map_err(|_| actix_web::error::ErrorInternalServerError("Server error"))?
    .map_err(|_| actix_web::error::ErrorInternalServerError("DB error"))?;

    Ok(HttpResponse::Ok().json(serde_json::json!({
        "filename": filename,
        "size_bytes": size_bytes,
    })))
}

fn sanitize_filename(name: &str) -> String {
    let base = name.rsplit(['/', '\\']).next().unwrap_or("").trim();
    let cleaned: String = base
        .chars()
        .filter(|c| c.is_ascii_alphanumeric() || matches!(c, '-' | '_' | '.'))
        .collect();
    let cleaned = cleaned.trim_matches('.').to_string();
    if cleaned.is_empty() || !cleaned.to_ascii_lowercase().ends_with(".flac") {
        default_filename()
    } else {
        cleaned
    }
}

fn default_filename() -> String {
    let secs = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(0);
    format!("recording-{secs}.flac")
}
