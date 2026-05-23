use actix_web::{HttpResponse, post};
use actix_web::cookie::{Cookie, SameSite, time::Duration};

#[post("/logout")]
pub async fn post_logout() -> HttpResponse {
    let mut session = Cookie::build("session", "")
        .http_only(true)
        .secure(true)
        .same_site(SameSite::Lax)
        .path("/")
        .max_age(Duration::ZERO)
        .finish();
    session.make_removal();

    let mut is_admin = Cookie::build("is_admin", "")
        .http_only(false)
        .secure(true)
        .same_site(SameSite::Lax)
        .path("/")
        .max_age(Duration::ZERO)
        .finish();
    is_admin.make_removal();

    HttpResponse::Ok()
        .cookie(session)
        .cookie(is_admin)
        .finish()
}
