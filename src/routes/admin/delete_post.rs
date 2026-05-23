use actix_web::{HttpResponse, delete, web};
use diesel::prelude::*;
use uuid::Uuid;

use crate::db::DbPool;
use crate::middleware::paseto_auth::AuthUser;
use crate::schema::{blog_posts, users};

#[delete("/posts/{id}")]
pub async fn delete_post(
    pool: web::Data<DbPool>,
    auth: web::ReqData<AuthUser>,
    post_id: web::Path<Uuid>,
) -> Result<HttpResponse, actix_web::Error> {
    let user_id = auth.into_inner().0;
    let post_id = post_id.into_inner();
    let pool_inner = pool.clone();

    let is_admin = web::block(move || {
        let mut conn = pool_inner.get().expect("db pool exhausted");
        users::table
            .filter(users::id.eq(user_id))
            .select(users::is_admin)
            .first::<bool>(&mut conn)
    })
    .await
    .map_err(|_| actix_web::error::ErrorInternalServerError("Server error"))?
    .map_err(|_| actix_web::error::ErrorInternalServerError("Database error"))?;

    if !is_admin {
        return Err(actix_web::error::ErrorForbidden("Admin access required"));
    }

    web::block(move || {
        let mut conn = pool.get().expect("db pool exhausted");
        diesel::delete(blog_posts::table.filter(blog_posts::id.eq(post_id))).execute(&mut conn)
    })
    .await
    .map_err(|_| actix_web::error::ErrorInternalServerError("Server error"))?
    .map_err(|_| actix_web::error::ErrorInternalServerError("Database error"))?;

    Ok(HttpResponse::NoContent().finish())
}
