use actix_web::{web, App, HttpServer};
mod routes;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(
                web::scope("/api")
                    .configure(routes::auth::configure)
            )
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}