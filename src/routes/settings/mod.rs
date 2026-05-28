use actix_web::web;

use crate::middleware::PasetoAuth;

pub mod change_password;
pub mod delete_profile_image;
pub mod get_settings;
pub mod toggle_2fa;
pub mod upload_profile_image;

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/settings")
            .wrap(PasetoAuth)
            .service(get_settings::get_settings)
            .service(toggle_2fa::toggle_2fa)
            .service(change_password::change_password)
            .service(upload_profile_image::upload_profile_image)
            .service(delete_profile_image::delete_profile_image),
    );
}
