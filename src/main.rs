use std::{env, sync::Arc};

use actix_web::{App, HttpServer};
use diesel::{r2d2::ConnectionManager, PgConnection};
use repositories::{postgres::{post_repository::PostRepository, user_repository::UserRepository}, traits::{post_repository::PostRepositoryTrait, user_repository::UserRepositoryTrait}};

mod routes;
mod handlers;
mod services;
mod repositories;
mod models;
mod schema;
mod lib;
mod extractors;

pub struct AppData {
    user_repository: Arc<dyn UserRepositoryTrait + Send + Sync>,
    post_repository: Arc<dyn PostRepositoryTrait + Send + Sync>,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    if let Err(err) = dotenvy::dotenv() {
        eprintln!("Failed to load .env file: {}", err);
    }

    let manager = ConnectionManager::<PgConnection>::new(env::var("DATABASE_URL").expect("DATABASE_URL must be set"));
    let pool = r2d2::Pool::builder()
        .max_size(5)
        .build(manager)
        .expect("Failed to create pool.");

    let app_data = actix_web::web::Data::<AppData>::new(AppData {
        user_repository: Arc::new(UserRepository::new(pool.clone())),
        post_repository: Arc::new(PostRepository::new(pool.clone())),
    });

    HttpServer::new(move || {
        App::new().
            app_data(app_data.clone()).
            configure(routes::main_routes::config)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await

}
