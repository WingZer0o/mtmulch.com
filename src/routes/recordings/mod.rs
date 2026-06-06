use actix_web::web;

use crate::middleware::PasetoAuth;

pub mod delete;
pub mod get;
pub mod list;
pub mod upload;

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/recordings")
            .wrap(PasetoAuth)
            .service(upload::upload)
            .service(list::list_recordings)
            .service(get::get_recording)
            .service(delete::delete_recording),
    );
}
