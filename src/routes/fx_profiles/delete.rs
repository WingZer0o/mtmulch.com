use actix_web::{HttpMessage, HttpRequest, HttpResponse, delete, web};
use diesel::prelude::*;
use uuid::Uuid;

use crate::db::DbPool;
use crate::middleware::AuthUser;
use crate::schema::fx_profiles;

#[delete("/{id}")]
pub async fn delete_fx_profile(
    pool: web::Data<DbPool>,
    req: HttpRequest,
    path: web::Path<Uuid>,
) -> actix_web::Result<HttpResponse> {
    let user_id = req.extensions().get::<AuthUser>().unwrap().0;
    let profile_id = path.into_inner();

    let deleted = web::block(move || {
        let mut conn = pool.get().expect("db pool exhausted");
        diesel::delete(
            fx_profiles::table
                .filter(fx_profiles::id.eq(profile_id))
                .filter(fx_profiles::user_id.eq(user_id)),
        )
        .execute(&mut conn)
    })
    .await
    .map_err(|_| actix_web::error::ErrorInternalServerError("Server error"))?
    .map_err(|_| actix_web::error::ErrorInternalServerError("DB error"))?;

    if deleted == 0 {
        return Err(actix_web::error::ErrorNotFound("Profile not found"));
    }

    Ok(HttpResponse::Ok().finish())
}
