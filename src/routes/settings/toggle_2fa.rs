use actix_web::{HttpMessage, HttpRequest, HttpResponse, post, web};
use diesel::prelude::*;
use serde::Deserialize;

use crate::db::DbPool;
use crate::middleware::AuthUser;
use crate::schema::users;

#[derive(Deserialize)]
struct Toggle2faRequest {
    enabled: bool,
}

#[post("/settings/2fa")]
pub async fn toggle_2fa(
    pool: web::Data<DbPool>,
    body: web::Json<Toggle2faRequest>,
    req: HttpRequest,
) -> Result<HttpResponse, actix_web::Error> {
    let user_id = req.extensions().get::<AuthUser>().unwrap().0;
    let pool_inner = pool.clone();
    let enabled = body.enabled;

    web::block(move || {
        let mut conn = pool_inner.get().expect("db pool exhausted");
        diesel::update(users::table.filter(users::id.eq(user_id)))
            .set(users::otp_enabled.eq(enabled))
            .execute(&mut conn)
    })
    .await
    .map_err(|_| actix_web::error::ErrorInternalServerError("Server error"))?
    .map_err(|_| actix_web::error::ErrorInternalServerError("Database error"))?;

    Ok(HttpResponse::Ok().finish())
}
