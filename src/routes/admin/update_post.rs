use actix_web::{HttpResponse, put, web};
use chrono::Utc;
use diesel::prelude::*;
use serde::Deserialize;
use uuid::Uuid;

use crate::db::DbPool;
use crate::email_queue::{EmailJob, EmailQueue};
use crate::middleware::paseto_auth::AuthUser;
use crate::models::blog_post::{BlogPost, UpdateBlogPost};
use crate::schema::{blog_posts, email_list, users};

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
    queue: web::Data<EmailQueue>,
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

    let publishing = body.published == Some(true);
    let changeset = UpdateBlogPost {
        title: body.title.clone(),
        slug: body.slug.clone(),
        content: body.content.clone(),
        published: body.published,
        updated_at: Utc::now(),
    };

    // Fetch the current published state and apply the update in one block.
    let pool_for_update = pool.clone();
    let (post, was_unpublished) = web::block(move || {
        let mut conn = pool_for_update.get().expect("db pool exhausted");

        let was_published: bool = blog_posts::table
            .filter(blog_posts::id.eq(post_id))
            .select(blog_posts::published)
            .first::<bool>(&mut conn)
            .map_err(|e| match e {
                diesel::result::Error::NotFound => diesel::result::Error::NotFound,
                e => e,
            })?;

        let post = diesel::update(blog_posts::table.filter(blog_posts::id.eq(post_id)))
            .set(&changeset)
            .returning(BlogPost::as_returning())
            .get_result::<BlogPost>(&mut conn)
            .map_err(|e| match e {
                diesel::result::Error::NotFound => diesel::result::Error::NotFound,
                e => e,
            })?;

        Ok::<_, diesel::result::Error>((post, !was_published))
    })
    .await
    .map_err(|_| actix_web::error::ErrorInternalServerError("Server error"))?
    .map_err(|e| match e {
        diesel::result::Error::NotFound => actix_web::error::ErrorNotFound("Post not found"),
        _ => actix_web::error::ErrorInternalServerError("Database error"),
    })?;

    if publishing && was_unpublished {
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

    Ok(HttpResponse::Ok().json(post))
}
