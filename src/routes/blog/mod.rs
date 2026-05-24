use actix_web::web;

use crate::middleware::PasetoAuth;

pub mod create_comment;
pub mod get_post;
pub mod list_comments;
pub mod list_posts;

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/blog")
            .service(list_posts::get_list_posts)
            .service(get_post::get_post)
            .service(list_comments::list_comments)
            .service(
                web::scope("")
                    .wrap(PasetoAuth)
                    .service(create_comment::create_comment),
            ),
    );
}
