use actix_cors::Cors;
use actix_web::{web, App, HttpServer};

mod db;
mod middleware;
mod models;
mod paseto;
mod routes;
mod schema;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenvy::dotenv().ok();

    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = db::create_pool(&database_url);

    HttpServer::new(move || {
        let cors = Cors::default()
            .allowed_origin("https://mtmulch.com")
            .allowed_origin("https://www.mtmulch.com")
            .allowed_methods(vec!["GET", "POST", "PUT", "DELETE", "OPTIONS"])
            .allowed_headers(vec![
                actix_web::http::header::CONTENT_TYPE,
                actix_web::http::header::AUTHORIZATION,
            ])
            .supports_credentials()
            .max_age(3600);

        App::new()
            .wrap(cors)
            .app_data(web::Data::new(pool.clone()))
            .service(
                web::scope("/api")
                    .configure(routes::auth::configure)
            )
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}
