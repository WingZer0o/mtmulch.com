use actix_web::web;

use crate::middleware::PasetoAuth;

pub mod login;
pub mod logout;
pub mod register;

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/auth")
            .service(login::post_login)
            .service(register::post_register)
            .service(
                web::scope("")
                    .wrap(PasetoAuth)
                    .service(logout::post_logout),
            ),
    );
}
