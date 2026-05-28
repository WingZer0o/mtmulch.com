use actix_web::{HttpResponse, get, web};
use chrono::{DateTime, Utc};
use diesel::prelude::*;
use serde::Serialize;
use uuid::Uuid;

use crate::db::DbPool;
use crate::schema::{blog_posts, post_comments, users};

#[derive(Queryable, Serialize)]
struct CommentResponse {
    id: Uuid,
    user_id: Uuid,
    author_email: String,
    content: String,
    created_at: DateTime<Utc>,
    has_profile_image: bool,
}

#[get("/posts/{slug}/comments")]
pub async fn list_comments(
    pool: web::Data<DbPool>,
    slug: web::Path<String>,
) -> Result<HttpResponse, actix_web::Error> {
    let slug = slug.into_inner();

    let comments = web::block(move || {
        let mut conn = pool.get().expect("db pool exhausted");

        let post_id: Uuid = blog_posts::table
            .filter(blog_posts::slug.eq(&slug).and(blog_posts::published.eq(true)))
            .select(blog_posts::id)
            .first(&mut conn)?;

        post_comments::table
            .inner_join(users::table.on(post_comments::user_id.eq(users::id)))
            .filter(post_comments::post_id.eq(post_id))
            .select((
                post_comments::id,
                post_comments::user_id,
                users::email,
                post_comments::content,
                post_comments::created_at,
                users::profile_image.is_not_null(),
            ))
            .order(post_comments::created_at.asc())
            .load::<CommentResponse>(&mut conn)
    })
    .await
    .map_err(|_| actix_web::error::ErrorInternalServerError("Server error"))?
    .map_err(|e| match e {
        diesel::result::Error::NotFound => actix_web::error::ErrorNotFound("Post not found"),
        _ => actix_web::error::ErrorInternalServerError("Database error"),
    })?;

    Ok(HttpResponse::Ok().json(comments))
}
