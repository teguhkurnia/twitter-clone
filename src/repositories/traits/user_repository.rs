use crate::models::user::{NewUser, User};

pub enum UserIdentifier{
    Username(String),
    Email(String),
    UsernameOrEmail { username: String, email: String },
}

#[async_trait::async_trait]
pub trait UserRepositoryTrait {
    async fn find_user(&self, indetifier: UserIdentifier) -> Option<User>;
    async fn create_user(&self, user: &NewUser) -> Result<User, String>;
}