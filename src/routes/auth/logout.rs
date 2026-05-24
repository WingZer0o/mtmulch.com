use actix_web::cookie::time::OffsetDateTime;
use actix_web::{HttpResponse, post};
use actix_web::cookie::{Cookie, SameSite, time::Duration};

fn removal_cookie(name: &'static str, http_only: bool) -> Cookie<'static> {
    let builder = Cookie::build(name, "")
        .http_only(http_only)
        .secure(true)
        .same_site(SameSite::None)
        .path("/")
        .max_age(Duration::ZERO)
        .expires(OffsetDateTime::now_utc() - Duration::days(1));
    builder.finish()
}

#[post("/logout")]
pub async fn post_logout() -> HttpResponse {
    HttpResponse::Ok()
        .cookie(removal_cookie("session", true))
        .cookie(removal_cookie("is_admin", false))
        .finish()
}
