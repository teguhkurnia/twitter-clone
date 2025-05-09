use crate::handlers::auth_handler;

pub struct AuthRoutes;

impl AuthRoutes {
  pub fn config(cfg: &mut actix_web::web::ServiceConfig) {
    cfg.service(actix_web::web::scope("/auth")
      .service(auth_handler::login)
      .service(auth_handler::register)
    );
  }
}
