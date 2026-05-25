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

        let post = blog_posts::table
            .filter(blog_posts::slug.eq(&slug).and(blog_posts::published.eq(true)))
            .select(BlogPost::as_select())
            .first::<BlogPost>(&mut conn)?;

        diesel::update(blog_posts::table.find(post.id))
            .set(blog_posts::views.eq(blog_posts::views + 1))
            .execute(&mut conn)?;

        Ok(post)
    })
    .await
    .map_err(|_| actix_web::error::ErrorInternalServerError("Server error"))?
    .map_err(|e: diesel::result::Error| match e {
        diesel::result::Error::NotFound => actix_web::error::ErrorNotFound("Post not found"),
        _ => actix_web::error::ErrorInternalServerError("Database error"),
    })?;

    Ok(HttpResponse::Ok().json(post))
}
