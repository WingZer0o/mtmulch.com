use actix_web::web;

use crate::middleware::PasetoAuth;

pub mod delete;
pub mod list;
pub mod save;

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/fx-profiles")
            .wrap(PasetoAuth)
            .service(list::list_fx_profiles)
            .service(save::save_fx_profile)
            .service(delete::delete_fx_profile),
    );
}
