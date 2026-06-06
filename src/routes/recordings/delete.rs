use actix_web::{HttpMessage, HttpRequest, HttpResponse, delete, web};
use azure_storage_blobs::prelude::ContainerClient;
use diesel::prelude::*;
use uuid::Uuid;

use crate::db::DbPool;
use crate::middleware::AuthUser;
use crate::schema::recordings;

#[delete("/{id}")]
pub async fn delete_recording(
    pool: web::Data<DbPool>,
    req: HttpRequest,
    path: web::Path<Uuid>,
    container: web::Data<ContainerClient>,
) -> actix_web::Result<HttpResponse> {
    let user_id = req.extensions().get::<AuthUser>().unwrap().0;
    let recording_id = path.into_inner();

    // Fetch the blob key, verifying the recording belongs to this user.
    let blob_key = {
        let pool = pool.clone();
        web::block(move || {
            let mut conn = pool.get().expect("db pool exhausted");
            recordings::table
                .filter(recordings::id.eq(recording_id))
                .filter(recordings::user_id.eq(user_id))
                .select(recordings::blob_key)
                .first::<String>(&mut conn)
                .optional()
        })
        .await
        .map_err(|_| actix_web::error::ErrorInternalServerError("Server error"))?
        .map_err(|_| actix_web::error::ErrorInternalServerError("DB error"))?
    };

    let Some(blob_key) = blob_key else {
        return Err(actix_web::error::ErrorNotFound("Recording not found"));
    };

    // Delete from blob storage first.
    container
        .blob_client(&blob_key)
        .delete()
        .await
        .map_err(|e| actix_web::error::ErrorInternalServerError(e.to_string()))?;

    // Remove the metadata row.
    web::block(move || {
        let mut conn = pool.get().expect("db pool exhausted");
        diesel::delete(
            recordings::table
                .filter(recordings::id.eq(recording_id))
                .filter(recordings::user_id.eq(user_id)),
        )
        .execute(&mut conn)
    })
    .await
    .map_err(|_| actix_web::error::ErrorInternalServerError("Server error"))?
    .map_err(|_| actix_web::error::ErrorInternalServerError("DB error"))?;

    Ok(HttpResponse::NoContent().finish())
}
