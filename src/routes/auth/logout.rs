use actix_web::{HttpResponse, post};
use actix_web::cookie::{Cookie, SameSite, time::Duration};

#[post("/logout")]
pub async fn post_logout() -> HttpResponse {
    let session = Cookie::build("session", "")
        .http_only(true)
        .secure(true)
        .same_site(SameSite::Lax)
        .path("/")
        .max_age(Duration::ZERO)
        .finish();

    HttpResponse::Ok().cookie(session).finish()
}
