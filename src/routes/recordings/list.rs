use actix_web::{HttpMessage, HttpRequest, HttpResponse, get, web};
use chrono::{DateTime, Utc};
use diesel::prelude::*;
use serde::Serialize;
use uuid::Uuid;

use crate::db::DbPool;
use crate::middleware::AuthUser;
use crate::schema::recordings;

#[derive(Serialize)]
struct RecordingMeta {
    id: Uuid,
    filename: String,
    size_bytes: i64,
    created_at: DateTime<Utc>,
}

#[get("")]
pub async fn list_recordings(
    pool: web::Data<DbPool>,
    req: HttpRequest,
) -> actix_web::Result<HttpResponse> {
    let user_id = req.extensions().get::<AuthUser>().unwrap().0;

    let rows = web::block(move || {
        let mut conn = pool.get().expect("db pool exhausted");
        recordings::table
            .filter(recordings::user_id.eq(user_id))
            .select((
                recordings::id,
                recordings::filename,
                recordings::size_bytes,
                recordings::created_at,
            ))
            .order(recordings::created_at.desc())
            .load::<(Uuid, String, i64, DateTime<Utc>)>(&mut conn)
    })
    .await
    .map_err(|_| actix_web::error::ErrorInternalServerError("Server error"))?
    .map_err(|_| actix_web::error::ErrorInternalServerError("DB error"))?;

    let metas: Vec<RecordingMeta> = rows
        .into_iter()
        .map(|(id, filename, size_bytes, created_at)| RecordingMeta {
            id,
            filename,
            size_bytes,
            created_at,
        })
        .collect();

    Ok(HttpResponse::Ok().json(metas))
}
