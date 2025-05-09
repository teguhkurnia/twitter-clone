use async_trait::async_trait;
use diesel::{r2d2::ConnectionManager, ExpressionMethods, PgConnection, QueryDsl, RunQueryDsl, SelectableHelper};
use r2d2::Pool;

use crate::{models::{self, post::Post}, repositories::traits::post_repository::PostRepositoryTrait, schema::posts::dsl};

pub struct PostRepository {
    database_pool: Pool<ConnectionManager<PgConnection>>,
}

impl PostRepository {
    pub fn new(database_pool: Pool<ConnectionManager<PgConnection>>) -> Self {
        PostRepository { database_pool }
    }
}

#[async_trait]
impl PostRepositoryTrait for PostRepository {
    async fn find_posts_by_user_id(
        &self,
        user_id_value: i32,
    ) -> Result<Vec<Post>, String> {
        let mut conn = self.database_pool.get().unwrap();

        let posts = dsl::posts.filter(dsl::user_id.eq(user_id_value))
            .select(models::post::Post::as_select())
            .load::<Post>(&mut conn)
            .map_err(|e| e.to_string())?;

        Ok(posts)
    }
}