use actix_web::{HttpResponse, get, web};
use diesel::prelude::*;
use serde::Serialize;
use uuid::Uuid;
use chrono::{DateTime, Utc};

use crate::db::DbPool;
use crate::schema::blog_posts;

#[derive(Serialize, Queryable)]
struct PostSummary {
    id: Uuid,
    title: String,
    slug: String,
    published: bool,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
    views: i64
}

#[get("/posts")]
pub async fn get_list_posts(pool: web::Data<DbPool>) -> Result<HttpResponse, actix_web::Error> {
    let posts = web::block(move || {
        let mut conn = pool.get().expect("db pool exhausted");
        blog_posts::table
            .filter(blog_posts::published.eq(true))
            .select((
                blog_posts::id,
                blog_posts::title,
                blog_posts::slug,
                blog_posts::published,
                blog_posts::created_at,
                blog_posts::updated_at,
                blog_posts::views
            ))
            .order(blog_posts::created_at.desc())
            .load::<PostSummary>(&mut conn)
    })
    .await
    .map_err(|_| actix_web::error::ErrorInternalServerError("Server error"))?
    .map_err(|_| actix_web::error::ErrorInternalServerError("Database error"))?;

    Ok(HttpResponse::Ok().json(posts))
}
