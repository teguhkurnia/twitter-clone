use async_trait::async_trait;

use crate::models::post::Post;

#[async_trait]
pub trait PostRepositoryTrait {
    async fn find_posts_by_user_id(&self, user_id: i32) -> Result<Vec<Post>, String>;
}