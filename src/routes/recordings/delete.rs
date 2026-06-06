use actix_web::{HttpMessage, HttpRequest, HttpResponse, delete, web};
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
) -> actix_web::Result<HttpResponse> {
    let user_id = req.extensions().get::<AuthUser>().unwrap().0;
    let recording_id = path.into_inner();

    let rows_deleted = web::block(move || {
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

    if rows_deleted == 0 {
        return Err(actix_web::error::ErrorNotFound("Recording not found"));
    }

    Ok(HttpResponse::NoContent().finish())
}
