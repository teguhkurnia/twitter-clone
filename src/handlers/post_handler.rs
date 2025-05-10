use actix_web::{get, post, web::{self, Json}, HttpResponse};
use actix_web_validation::Validated;

use crate::{extractors::auth_extractor::Authenticated, models::dto::create_post_dto::CreatePostDto, AppData};

#[get("")]
pub async fn get_posts(
  app_data: web::Data<AppData>,
  auth: Authenticated,
) -> HttpResponse {
  print!("User ID: {}", auth.id);
  let posts = app_data.post_repository.find_posts_by_user_id(auth.id).await.unwrap();
  HttpResponse::Ok().json(posts)
}

#[post("")]
pub async fn create_post(
  app_data: web::Data<AppData>,
  auth: Authenticated,
  Validated(Json(payload)): Validated<Json<CreatePostDto>>,
) -> HttpResponse {
  let post = app_data.post_repository.create_post(auth.id, payload).await.unwrap();
  HttpResponse::Created().json(post)
}

#[get("{post_id}/replies")]
pub async fn get_replies(
  app_data: web::Data<AppData>,
  auth: Authenticated,
  post_id: web::Path<i32>,
) -> HttpResponse {
  let replies = app_data.post_repository.find_post_replies(auth.id, post_id.into_inner()).await.unwrap();
  HttpResponse::Ok().json(replies)
}