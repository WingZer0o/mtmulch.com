use actix_web::{HttpMessage, HttpRequest, HttpResponse, post, web};
use chrono::Utc;
use diesel::prelude::*;
use uuid::Uuid;

use crate::db::DbPool;
use crate::middleware::AuthUser;
use crate::models::comment::{CreateCommentRequest, NewComment};
use crate::schema::{blog_posts, post_comments};

#[post("/posts/{slug}/comments")]
pub async fn create_comment(
    pool: web::Data<DbPool>,
    slug: web::Path<String>,
    req: HttpRequest,
    body: web::Json<CreateCommentRequest>,
) -> Result<HttpResponse, actix_web::Error> {
    let user_id = req.extensions().get::<AuthUser>().unwrap().0;
    let slug = slug.into_inner();
    let content = body.into_inner().content;

    if content.trim().is_empty() {
        return Err(actix_web::error::ErrorBadRequest("Comment cannot be empty"));
    }

    let comment = web::block(move || {
        let mut conn = pool.get().expect("db pool exhausted");

        let post_id: Uuid = blog_posts::table
            .filter(blog_posts::slug.eq(&slug).and(blog_posts::published.eq(true)))
            .select(blog_posts::id)
            .first(&mut conn)?;

        let new_comment = NewComment {
            id: Uuid::new_v4(),
            post_id,
            user_id,
            content: content.trim().to_string(),
            created_at: Utc::now(),
        };

        diesel::insert_into(post_comments::table)
            .values(&new_comment)
            .returning((
                post_comments::id,
                post_comments::post_id,
                post_comments::user_id,
                post_comments::content,
                post_comments::created_at,
            ))
            .get_result::<(Uuid, Uuid, Uuid, String, chrono::DateTime<Utc>)>(&mut conn)
    })
    .await
    .map_err(|_| actix_web::error::ErrorInternalServerError("Server error"))?
    .map_err(|e| match e {
        diesel::result::Error::NotFound => actix_web::error::ErrorNotFound("Post not found"),
        _ => actix_web::error::ErrorInternalServerError("Database error"),
    })?;

    Ok(HttpResponse::Created().json(serde_json::json!({
        "id": comment.0,
        "post_id": comment.1,
        "user_id": comment.2,
        "content": comment.3,
        "created_at": comment.4,
    })))
}
