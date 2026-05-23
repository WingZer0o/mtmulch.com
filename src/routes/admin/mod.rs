use actix_web::web;

use crate::middleware::PasetoAuth;

pub mod create_post;
pub mod delete_post;
pub mod list_posts;
pub mod update_post;

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/admin")
            .wrap(PasetoAuth)
            .service(list_posts::get_list_posts)
            .service(create_post::post_create)
            .service(update_post::put_update)
            .service(delete_post::delete_post),
    );
}
