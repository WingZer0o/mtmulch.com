use actix_cors::Cors;
use actix_web::{web, App, HttpServer};

mod db;
mod email_queue;
mod helpers;
mod middleware;
mod models;
mod routes;
mod schema;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenvy::dotenv().ok();

    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let resend_api_key = std::env::var("RESEND_API_KEY").expect("RESEND_API_KEY must be set");

    let pool = db::create_pool(&database_url);
    let queue = email_queue::start_worker(resend_api_key);

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
            .app_data(web::Data::new(queue.clone()))
            .service(
                web::scope("/api")
                    .configure(routes::auth::configure)
                    .configure(routes::settings::configure)
                    .configure(routes::blog::configure)
                    .configure(routes::admin::configure)
                    .configure(routes::users::configure),
            )
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}
