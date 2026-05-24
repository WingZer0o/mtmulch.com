use actix_web::{HttpResponse, post};

#[post("/logout")]
pub async fn post_logout() -> HttpResponse {
    HttpResponse::Ok().finish()
}
