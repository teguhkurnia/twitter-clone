use actix_web::{post, web::Json, HttpResponse};
use actix_web_validation::Validated;
use garde::Validate;
use serde::Deserialize;

use crate::{services::auth_service::AuthService, AppData};

#[derive(Deserialize, Validate, Debug)]
struct LoginRequest {
    #[garde(length(min = 1))]
    username: String,
    #[garde(length(min = 1))]
    password: String,
}

#[derive(Deserialize, Validate, Debug)]
pub struct RegisterRequest {
    #[garde(length(min = 1), alphanumeric)]
    pub username: String,
    #[garde(length(min = 1))]
    pub password: String,
    #[garde(length(min = 1), email)]
    pub email: String,
}

#[post("/login")]
pub async fn login(
    data: actix_web::web::Data<AppData>,
    Validated(Json(payload)) : Validated<Json<LoginRequest>>
) -> HttpResponse {
    match AuthService::new(data.user_repository.clone()).login(payload.username, payload.password).await {
        Ok(user) => {
            HttpResponse::Ok().json(user)
        }
        Err(err) => {
            println!("Error: {:?}", err);
            HttpResponse::BadRequest().body(format!("Error: {:?}", err))
        }
    }
}

#[post("/register")]
pub async fn register(
    data: actix_web::web::Data<AppData>,
    Validated(Json(payload)) : Validated<Json<RegisterRequest>>
) -> HttpResponse {
    match AuthService::new(data.user_repository.clone()).register(payload).await {
        Ok(user) => {
            HttpResponse::Ok().json(user)
        }
        Err(err) => {
            println!("Error: {:?}", err);
            HttpResponse::BadRequest().body(format!("Error: {:?}", err))
        }
    }
}