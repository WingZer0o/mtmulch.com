use actix_web::{HttpMessage, HttpRequest, HttpResponse, delete, web};
use diesel::prelude::*;

use crate::db::DbPool;
use crate::middleware::AuthUser;
use crate::schema::users;

#[delete("/profile-image")]
pub async fn delete_profile_image(
    req: HttpRequest,
    pool: web::Data<DbPool>,
) -> Result<HttpResponse, actix_web::Error> {
    let user_id = req.extensions().get::<AuthUser>().unwrap().0;
    let pool_inner = pool.clone();

    web::block(move || {
        let mut conn = pool_inner.get().expect("db pool exhausted");
        diesel::update(users::table.filter(users::id.eq(user_id)))
            .set((
                users::profile_image.eq(None::<Vec<u8>>),
                users::profile_image_content_type.eq(None::<String>),
                users::updated_at.eq(chrono::Utc::now()),
            ))
            .execute(&mut conn)
    })
    .await
    .map_err(|_| actix_web::error::ErrorInternalServerError("Server error"))?
    .map_err(|_| actix_web::error::ErrorInternalServerError("Database error"))?;

    Ok(HttpResponse::Ok().finish())
}
