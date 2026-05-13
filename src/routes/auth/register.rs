use actix_web::{HttpResponse, post};

#[post("/register")]
async fn post_register(req_body: String) -> Result<HttpResponse, actix_web::Error> {
    Ok(HttpResponse::Ok().body(req_body))
}