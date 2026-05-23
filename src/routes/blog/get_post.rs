use actix_web::{HttpResponse, get, web};
use diesel::prelude::*;

use crate::db::DbPool;
use crate::models::blog_post::BlogPost;
use crate::schema::blog_posts;

#[get("/posts/{slug}")]
pub async fn get_post(
    pool: web::Data<DbPool>,
    slug: web::Path<String>,
) -> Result<HttpResponse, actix_web::Error> {
    let slug = slug.into_inner();

    let post = web::block(move || {
        let mut conn = pool.get().expect("db pool exhausted");
        blog_posts::table
            .filter(blog_posts::slug.eq(&slug).and(blog_posts::published.eq(true)))
            .select(BlogPost::as_select())
            .first::<BlogPost>(&mut conn)
    })
    .await
    .map_err(|_| actix_web::error::ErrorInternalServerError("Server error"))?
    .map_err(|e| match e {
        diesel::result::Error::NotFound => actix_web::error::ErrorNotFound("Post not found"),
        _ => actix_web::error::ErrorInternalServerError("Database error"),
    })?;

    Ok(HttpResponse::Ok().json(post))
}
