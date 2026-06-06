use actix_web::{HttpMessage, HttpRequest, HttpResponse, get, web};
use async_stream::stream;
use azure_storage_blobs::prelude::ContainerClient;
use diesel::prelude::*;
use futures_util::StreamExt;
use uuid::Uuid;

use crate::db::DbPool;
use crate::middleware::AuthUser;
use crate::schema::recordings;

#[get("/{id}")]
pub async fn get_recording(
    pool: web::Data<DbPool>,
    req: HttpRequest,
    path: web::Path<Uuid>,
    container: web::Data<ContainerClient>,
) -> actix_web::Result<HttpResponse> {
    let user_id = req.extensions().get::<AuthUser>().unwrap().0;
    let recording_id = path.into_inner();

    let (filename, blob_key) = web::block(move || {
        let mut conn = pool.get().expect("db pool exhausted");
        recordings::table
            .filter(recordings::id.eq(recording_id))
            .filter(recordings::user_id.eq(user_id))
            .select((recordings::filename, recordings::blob_key))
            .first::<(String, String)>(&mut conn)
    })
    .await
    .map_err(|_| actix_web::error::ErrorInternalServerError("Server error"))?
    .map_err(|_| actix_web::error::ErrorNotFound("Recording not found"))?;

    // `get()` returns a Pageable (chunked download); flatten each chunk's
    // ResponseBody stream into a single byte stream for the HTTP response.
    let mut page_stream = container.blob_client(&blob_key).get().into_stream();
    let byte_stream = stream! {
        while let Some(page_result) = page_stream.next().await {
            match page_result {
                Ok(page) => {
                    let mut data = page.data;
                    while let Some(chunk) = data.next().await {
                        yield chunk.map_err(|e| actix_web::error::ErrorInternalServerError(e.to_string()));
                    }
                }
                Err(e) => {
                    yield Err(actix_web::error::ErrorInternalServerError(e.to_string()));
                    return;
                }
            }
        }
    };

    Ok(HttpResponse::Ok()
        .content_type("audio/flac")
        .insert_header((
            "Content-Disposition",
            format!("inline; filename=\"{filename}\""),
        ))
        .streaming(byte_stream))
}
