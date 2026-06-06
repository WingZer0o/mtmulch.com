use actix_web::{HttpMessage, HttpRequest, HttpResponse, get, web};
use chrono::{DateTime, Utc};
use diesel::prelude::*;
use serde::Serialize;
use uuid::Uuid;

use crate::db::DbPool;
use crate::middleware::AuthUser;
use crate::schema::fx_profiles;

#[derive(Serialize)]
struct ProfileMeta {
    id: Uuid,
    name: String,
    settings: String,
    created_at: DateTime<Utc>,
}

#[get("")]
pub async fn list_fx_profiles(
    pool: web::Data<DbPool>,
    req: HttpRequest,
) -> actix_web::Result<HttpResponse> {
    let user_id = req.extensions().get::<AuthUser>().unwrap().0;

    let rows = web::block(move || {
        let mut conn = pool.get().expect("db pool exhausted");
        fx_profiles::table
            .filter(fx_profiles::user_id.eq(user_id))
            .select((
                fx_profiles::id,
                fx_profiles::name,
                fx_profiles::settings,
                fx_profiles::created_at,
            ))
            .order(fx_profiles::created_at.desc())
            .load::<(Uuid, String, String, DateTime<Utc>)>(&mut conn)
    })
    .await
    .map_err(|_| actix_web::error::ErrorInternalServerError("Server error"))?
    .map_err(|_| actix_web::error::ErrorInternalServerError("DB error"))?;

    let profiles: Vec<ProfileMeta> = rows
        .into_iter()
        .map(|(id, name, settings, created_at)| ProfileMeta { id, name, settings, created_at })
        .collect();

    Ok(HttpResponse::Ok().json(profiles))
}
