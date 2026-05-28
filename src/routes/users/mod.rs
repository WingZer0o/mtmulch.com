use actix_web::web;

pub mod get_profile_image;

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/users")
            .service(get_profile_image::get_profile_image),
    );
}
