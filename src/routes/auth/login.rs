use actix_web::{HttpResponse, post};

#[post("/login")]
async fn post_login(req_body: String) -> Result<HttpResponse, actix_web::Error> {
    Ok(HttpResponse::Ok().body(req_body))
}