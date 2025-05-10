use async_trait::async_trait;

use crate::models::{dto::create_post_dto::CreatePostDto, post::Post};

#[async_trait]
pub trait PostRepositoryTrait {
    async fn find_posts_by_user_id(&self, user_id: i32) -> Result<Vec<Post>, String>;
    async fn find_post_replies(&self, user_id: i32, post_id: i32) -> Result<Vec<Post>, String>;
    async fn create_post(&self, user_id: i32, payload: CreatePostDto) -> Result<Post, String>;
}