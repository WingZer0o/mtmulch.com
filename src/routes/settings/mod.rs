use actix_web::web;

use crate::middleware::PasetoAuth;

pub mod get_settings;
pub mod toggle_2fa;

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("")
            .wrap(PasetoAuth)
            .service(get_settings::get_settings)
            .service(toggle_2fa::toggle_2fa),
    );
}
