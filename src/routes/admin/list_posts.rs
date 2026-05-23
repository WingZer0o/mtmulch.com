use actix_web::{HttpResponse, get, web};
use diesel::prelude::*;

use crate::db::DbPool;
use crate::middleware::paseto_auth::AuthUser;
use crate::models::blog_post::BlogPost;
use crate::schema::{blog_posts, users};

#[get("/posts")]
pub async fn get_list_posts(
    pool: web::Data<DbPool>,
    auth: web::ReqData<AuthUser>,
) -> Result<HttpResponse, actix_web::Error> {
    let user_id = auth.into_inner().0;
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

    let posts = web::block(move || {
        let mut conn = pool.get().expect("db pool exhausted");
        blog_posts::table
            .select(BlogPost::as_select())
            .order(blog_posts::created_at.desc())
            .load::<BlogPost>(&mut conn)
    })
    .await
    .map_err(|_| actix_web::error::ErrorInternalServerError("Server error"))?
    .map_err(|_| actix_web::error::ErrorInternalServerError("Database error"))?;

    Ok(HttpResponse::Ok().json(posts))
}
