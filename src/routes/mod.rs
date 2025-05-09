mod auth;
mod post;

pub mod main_routes {
    use actix_web::web;

    use super::{auth, post};

    pub fn config(cfg: &mut web::ServiceConfig) {
        cfg.service(
            web::scope("/api/v1")
                .configure(auth::AuthRoutes::config)
                .configure(post::PostRoutes::config)
        );
    }
}