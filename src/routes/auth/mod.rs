use actix_web::web;

pub mod login;
pub mod logout;
pub mod register;
pub mod unlock_account;
pub mod verify_email;
pub mod verify_otp;

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/auth")
            .service(login::post_login)
            .service(register::post_register)
            .service(verify_otp::post_verify_otp)
            .service(verify_email::get_verify_email)
            .service(unlock_account::get_unlock_account)
            .service(logout::post_logout),
    );
}
