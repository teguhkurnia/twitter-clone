use std::env;

use hmac::{Hmac, Mac};
use serde::{Deserialize, Serialize};
use sha2::Sha256;
use jwt::{SignWithKey, VerifyWithKey};

use crate::models::user::User;

#[derive(Serialize, Deserialize)]
pub struct Claims {
  pub id: i32,
  pub username: String,
  pub exp: usize,
}

pub fn create_jwt(user: &User) -> Result<String, String> {
  let key = Hmac::<Sha256>::new_from_slice(env::var("JWT_SECRET").expect("JWT_SECRET must be set").as_bytes())
    .expect("HMAC can take key of any size");

  let expired_at  = chrono::Utc::now()
    .checked_add_signed(chrono::Duration::days(1))
    .expect("valid timestamp")
    .timestamp();

  let claims = Claims {
    id: user.id,
    username: user.username.clone(),
    exp: expired_at as usize,
  };

  match claims.sign_with_key(&key) {
    Ok(token) => Ok(token), 
    Err(_) => Err("Failed to sign JWT".to_string()),
  }
}

pub fn verify_jwt(token: &str) -> Result<Claims, String> {
  let key = Hmac::<Sha256>::new_from_slice(env::var("JWT_SECRET").expect("JWT_SECRET must be set").as_bytes())
    .expect("HMAC can take key of any size");

  let claims = match token.verify_with_key(&key) {
    Ok(claims) => Ok(claims),
    Err(_) => Err("Failed to verify JWT".to_string()),
  };
  claims
}