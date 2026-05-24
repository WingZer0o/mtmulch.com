use actix_web::{HttpResponse, post};
use actix_web::cookie::{Cookie, SameSite, time::Duration};

fn removal_cookie(name: &'static str) -> Cookie<'static> {
    let domain = std::env::var("COOKIE_DOMAIN").ok();
    let builder = Cookie::build(name, "")
        .http_only(false)
        .secure(true)
        .same_site(SameSite::None)
        .path("/")
        .max_age(Duration::ZERO);
    let mut cookie = match domain {
        Some(d) => builder.domain(d).finish(),
        None => builder.finish(),
    };
    cookie.make_removal();
    cookie
}

#[post("/logout")]
pub async fn post_logout() -> HttpResponse {
    HttpResponse::Ok()
        .cookie(removal_cookie("session"))
        .cookie(removal_cookie("is_admin"))
        .finish()
}
