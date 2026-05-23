use actix_web::{HttpResponse, put, web};
use chrono::Utc;
use diesel::prelude::*;
use serde::Deserialize;
use uuid::Uuid;

use crate::db::DbPool;
use crate::middleware::paseto_auth::AuthUser;
use crate::models::blog_post::{BlogPost, UpdateBlogPost};
use crate::schema::{blog_posts, users};

#[derive(Deserialize)]
pub struct UpdatePostRequest {
    pub title: Option<String>,
    pub slug: Option<String>,
    pub content: Option<String>,
    pub published: Option<bool>,
}

#[put("/posts/{id}")]
pub async fn put_update(
    pool: web::Data<DbPool>,
    auth: web::ReqData<AuthUser>,
    post_id: web::Path<Uuid>,
    body: web::Json<UpdatePostRequest>,
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

    let changeset = UpdateBlogPost {
        title: body.title.clone(),
        slug: body.slug.clone(),
        content: body.content.clone(),
        published: body.published,
        updated_at: Utc::now(),
    };

    let post = web::block(move || {
        let mut conn = pool.get().expect("db pool exhausted");
        diesel::update(blog_posts::table.filter(blog_posts::id.eq(post_id)))
            .set(&changeset)
            .returning(BlogPost::as_returning())
            .get_result::<BlogPost>(&mut conn)
    })
    .await
    .map_err(|_| actix_web::error::ErrorInternalServerError("Server error"))?
    .map_err(|e| match e {
        diesel::result::Error::NotFound => actix_web::error::ErrorNotFound("Post not found"),
        _ => actix_web::error::ErrorInternalServerError("Database error"),
    })?;

    Ok(HttpResponse::Ok().json(post))
}
