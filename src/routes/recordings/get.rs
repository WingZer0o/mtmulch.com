use actix_web::{HttpMessage, HttpRequest, HttpResponse, get, web};
use diesel::prelude::*;
use uuid::Uuid;

use crate::db::DbPool;
use crate::middleware::AuthUser;
use crate::schema::recordings;

#[get("/{id}")]
pub async fn get_recording(
    pool: web::Data<DbPool>,
    req: HttpRequest,
    path: web::Path<Uuid>,
) -> actix_web::Result<HttpResponse> {
    let user_id = req.extensions().get::<AuthUser>().unwrap().0;
    let recording_id = path.into_inner();

    let result = web::block(move || {
        let mut conn = pool.get().expect("db pool exhausted");
        recordings::table
            .filter(recordings::id.eq(recording_id))
            .filter(recordings::user_id.eq(user_id))
            .select((recordings::filename, recordings::data))
            .first::<(String, Vec<u8>)>(&mut conn)
    })
    .await
    .map_err(|_| actix_web::error::ErrorInternalServerError("Server error"))?
    .map_err(|_| actix_web::error::ErrorNotFound("Recording not found"))?;

    let (filename, data) = result;

    Ok(HttpResponse::Ok()
        .content_type("audio/flac")
        .insert_header((
            "Content-Disposition",
            format!("inline; filename=\"{filename}\""),
        ))
        .body(data))
}
