use actix_web::{HttpMessage, HttpRequest, HttpResponse, post, web};
use diesel::prelude::*;
use serde::Deserialize;
use uuid::Uuid;

use crate::db::DbPool;
use crate::middleware::AuthUser;
use crate::models::fx_profile::NewFxProfile;
use crate::schema::fx_profiles;

#[derive(Deserialize)]
pub struct SaveProfileRequest {
    pub name: String,
    pub settings: String,
}

#[post("")]
pub async fn save_fx_profile(
    pool: web::Data<DbPool>,
    req: HttpRequest,
    body: web::Json<SaveProfileRequest>,
) -> actix_web::Result<HttpResponse> {
    let user_id = req.extensions().get::<AuthUser>().unwrap().0;
    let name = body.name.trim().to_string();

    if name.is_empty() {
        return Err(actix_web::error::ErrorBadRequest("Profile name is required"));
    }

    let new_profile = NewFxProfile {
        id: Uuid::new_v4(),
        user_id,
        name,
        settings: body.settings.clone(),
    };

    web::block(move || {
        let mut conn = pool.get().expect("db pool exhausted");
        diesel::insert_into(fx_profiles::table)
            .values(&new_profile)
            .execute(&mut conn)
    })
    .await
    .map_err(|_| actix_web::error::ErrorInternalServerError("Server error"))?
    .map_err(|_| actix_web::error::ErrorInternalServerError("DB error"))?;

    Ok(HttpResponse::Created().finish())
}
