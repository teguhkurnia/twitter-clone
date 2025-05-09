use std::sync::Arc;

use serde::{Deserialize, Serialize};

use crate::{handlers::auth_handler::RegisterRequest, lib, models::user, repositories::traits::user_repository::{UserIdentifier, UserRepositoryTrait}};

#[derive(Deserialize, Serialize)]
pub struct AuthResponse {
  username: String,
  email: String,
  profile_picture: String,
  token: String,
}

impl AuthResponse {
  pub fn from_user(user: &user::User, token: String) -> Self {
    AuthResponse {
      username: user.username.clone(),
      email: user.email.clone(),
      profile_picture: user.profile_picture.clone().unwrap_or("/default.png".to_string()),
      token,
    }
  }
}

pub struct AuthService {
  user_repository: Arc<dyn UserRepositoryTrait + Send + Sync>,
}

impl AuthService {
  pub fn new(user_repository: Arc<dyn UserRepositoryTrait + Send + Sync>) -> Self {
    AuthService { user_repository }
  }

  pub async fn login(&self, username: String, password: String) -> Result<AuthResponse, String> {
    let user = self.user_repository.find_user(UserIdentifier::Username(username.clone())).await;
    match user {
      Some(user) => {
        if bcrypt::verify(password, &user.password).unwrap() {
          // Generate a JWT token here (omitted for brevity)
          let token = lib::jwt::create_jwt(&user).unwrap();

          return Ok(AuthResponse::from_user(&user, token));
        } else {
          return Err("Invalid username or password".to_string());
        }
      },
      None => Err("Invalid username or password".to_string()),
    }
  }

  pub async fn register(&self, data: RegisterRequest) -> Result<AuthResponse, String> {
    // Check if the username already exists
    let existing_user = self.user_repository.find_user(UserIdentifier::UsernameOrEmail { username: data.username.clone(), email: data.email.clone() }).await;

    match existing_user {
      Some(_) => return Err("User already exists".to_string()),
      None => {}
    }

    let new_user = user::NewUser {
      username: data.username,
      password: bcrypt::hash(data.password, bcrypt::DEFAULT_COST).unwrap(),
      email: data.email
    };

    let result = self.user_repository.create_user(&new_user).await;

    match result {
      Ok(user) => return Ok(AuthResponse::from_user(&user, lib::jwt::create_jwt(&user).unwrap())),
      Err(e) => {
        println!("Error creating user: {}", e);
        return Err("Error creating user".to_string());
      }
        
    }
  }
}