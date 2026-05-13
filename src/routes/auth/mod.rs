use actix_web::web;

pub mod login;
pub mod register;

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/auth")
            .service(login::post_login)
            .service(register::post_register),
    );
}