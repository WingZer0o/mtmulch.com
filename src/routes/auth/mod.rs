use actix_web::web;

use crate::middleware::PasetoAuth;

pub mod login;
pub mod logout;
pub mod register;
pub mod verify_email;
pub mod verify_otp;

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/auth")
            .service(login::post_login)
            .service(register::post_register)
            .service(verify_otp::post_verify_otp)
            .service(verify_email::get_verify_email)
            .service(
                web::scope("")
                    .service(logout::post_logout),
            ),
    );
}
