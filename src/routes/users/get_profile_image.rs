use actix_web::{HttpResponse, get, web};
use base64::Engine;
use diesel::prelude::*;
use serde::Serialize;
use uuid::Uuid;

use crate::db::DbPool;
use crate::schema::users;

#[derive(Serialize)]
struct ProfileImageResponse {
    profile_image: String,
}

#[get("/{user_id}/profile-image")]
pub async fn get_profile_image(
    pool: web::Data<DbPool>,
    user_id: web::Path<Uuid>,
) -> Result<HttpResponse, actix_web::Error> {
    let user_id = user_id.into_inner();

    let result = web::block(move || {
        let mut conn = pool.get().expect("db pool exhausted");
        users::table
            .filter(users::id.eq(user_id))
            .select((users::profile_image, users::profile_image_content_type))
            .first::<(Option<Vec<u8>>, Option<String>)>(&mut conn)
    })
    .await
    .map_err(|_| actix_web::error::ErrorInternalServerError("Server error"))?
    .map_err(|_| actix_web::error::ErrorNotFound("User not found"))?;

    match result {
        (Some(bytes), Some(ct)) => {
            let profile_image = format!(
                "data:{};base64,{}",
                ct,
                base64::engine::general_purpose::STANDARD.encode(&bytes)
            );
            Ok(HttpResponse::Ok().json(ProfileImageResponse { profile_image }))
        }
        _ => Ok(HttpResponse::NotFound().finish()),
    }
}
