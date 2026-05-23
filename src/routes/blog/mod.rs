use actix_web::web;

pub mod get_post;
pub mod list_posts;

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/blog")
            .service(list_posts::get_list_posts)
            .service(get_post::get_post),
    );
}
