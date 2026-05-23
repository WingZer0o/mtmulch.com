use actix_web::{HttpResponse, post, web};
use chrono::Utc;
use diesel::prelude::*;
use serde::Deserialize;
use uuid::Uuid;

use crate::db::DbPool;
use crate::middleware::paseto_auth::AuthUser;
use crate::models::blog_post::{BlogPost, NewBlogPost};
use crate::schema::{blog_posts, users};

#[derive(Deserialize)]
pub struct CreatePostRequest {
    pub title: String,
    pub slug: String,
    pub content: String,
    pub published: bool,
}

#[post("/posts")]
pub async fn post_create(
    pool: web::Data<DbPool>,
    auth: web::ReqData<AuthUser>,
    body: web::Json<CreatePostRequest>,
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

    let now = Utc::now();
    let new_post = NewBlogPost {
        id: Uuid::new_v4(),
        title: body.title.clone(),
        slug: body.slug.clone(),
        content: body.content.clone(),
        published: body.published,
        author_id: user_id,
        created_at: now,
        updated_at: now,
    };

    let post = web::block(move || {
        let mut conn = pool.get().expect("db pool exhausted");
        diesel::insert_into(blog_posts::table)
            .values(&new_post)
            .returning(BlogPost::as_returning())
            .get_result::<BlogPost>(&mut conn)
    })
    .await
    .map_err(|_| actix_web::error::ErrorInternalServerError("Server error"))?
    .map_err(|_| actix_web::error::ErrorInternalServerError("Database error"))?;

    Ok(HttpResponse::Created().json(post))
}
