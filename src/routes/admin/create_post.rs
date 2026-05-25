use actix_web::{HttpResponse, post, web};
use chrono::Utc;
use diesel::prelude::*;
use serde::Deserialize;
use uuid::Uuid;

use crate::db::DbPool;
use crate::email_queue::{EmailJob, EmailQueue};
use crate::middleware::paseto_auth::AuthUser;
use crate::models::blog_post::{BlogPost, NewBlogPost};
use crate::schema::{blog_posts, email_list, users};

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
    queue: web::Data<EmailQueue>,
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
    let published = body.published;
    let new_post = NewBlogPost {
        id: Uuid::new_v4(),
        title: body.title.clone(),
        slug: body.slug.clone(),
        content: body.content.clone(),
        published,
        author_id: user_id,
        created_at: now,
        updated_at: now,
    };

    let pool_for_insert = pool.clone();
    let post = web::block(move || {
        let mut conn = pool_for_insert.get().expect("db pool exhausted");
        diesel::insert_into(blog_posts::table)
            .values(&new_post)
            .returning(BlogPost::as_returning())
            .get_result::<BlogPost>(&mut conn)
    })
    .await
    .map_err(|_| actix_web::error::ErrorInternalServerError("Server error"))?
    .map_err(|_| actix_web::error::ErrorInternalServerError("Database error"))?;

    if published {
        let title = post.title.clone();
        let slug = post.slug.clone();
        let pool_notify = pool.clone();

        let subscribers: Vec<String> = web::block(move || {
            let mut conn = pool_notify.get().expect("db pool exhausted");
            email_list::table
                .filter(email_list::verified.eq(true))
                .select(email_list::email)
                .load::<String>(&mut conn)
        })
        .await
        .map_err(|_| actix_web::error::ErrorInternalServerError("Server error"))?
        .map_err(|_| actix_web::error::ErrorInternalServerError("Database error"))?;

        let domain = std::env::var("APP_DOMAIN").unwrap_or_else(|_| "http://localhost:3000".to_string());
        let link = format!("{}/blog/{}", domain, slug);

        for email in subscribers {
            let _ = queue.send(EmailJob::NewBlogPost {
                to: email,
                title: title.clone(),
                link: link.clone(),
            });
        }
    }

    Ok(HttpResponse::Created().json(post))
}
