use actix_web::web;

use crate::handlers::post_handler;

pub struct PostRoutes;

impl PostRoutes {
    pub fn config(cfg: &mut web::ServiceConfig) {
        cfg.service(
            web::scope("/posts")
                .service(post_handler::get_posts)
                .service(post_handler::get_replies)
                .service(post_handler::create_post)
        );
    }
}