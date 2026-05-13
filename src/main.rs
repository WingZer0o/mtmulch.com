use actix_web::{web, App, HttpServer};

mod db;
mod models;
mod routes;
mod schema;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenvy::dotenv().ok();

    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = db::create_pool(&database_url);

    HttpServer::new(move || {
        App::new()
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
