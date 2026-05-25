use actix_web::web;

use crate::middleware::PasetoAuth;

pub mod confirm_unsubscribe;
pub mod create_comment;
pub mod get_post;
pub mod list_comments;
pub mod list_posts;
pub mod subscribe;
pub mod unsubscribe;
pub mod verify_subscription;

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/blog")
            .service(list_posts::get_list_posts)
            .service(get_post::get_post)
            .service(list_comments::list_comments)
            .service(subscribe::subscribe)
            .service(unsubscribe::unsubscribe)
            .service(verify_subscription::verify_subscription)
            .service(confirm_unsubscribe::confirm_unsubscribe)
            .service(
                web::scope("")
                    .wrap(PasetoAuth)
                    .service(create_comment::create_comment),
            ),
    );
}
