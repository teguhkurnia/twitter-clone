use async_trait::async_trait;
use diesel::{associations::HasTable, dsl::insert_into, r2d2::ConnectionManager, ExpressionMethods, PgConnection, QueryDsl, RunQueryDsl, SelectableHelper};
use r2d2::Pool;

use crate::{models::{self, dto::create_post_dto::CreatePostDto, post::{NewPost, Post}}, repositories::traits::post_repository::PostRepositoryTrait, schema::posts::{self, dsl}};

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
            .filter(dsl::parent_id.is_null())
            .filter(dsl::is_deleted.eq(false))
            .order(dsl::created_at.desc())
            .limit(10)
            .offset(0)
            .select(models::post::Post::as_select())
            .load::<Post>(&mut conn)
            .map_err(|e| e.to_string())?;


        Ok(posts)
    }

    async fn find_post_replies(
        &self,
        user_id_value: i32,
        post_id_value: i32,
    ) -> Result<Vec<Post>, String> {
        let mut conn = self.database_pool.get().unwrap();

        let posts = dsl::posts.filter(dsl::parent_id.eq(post_id_value))
            .filter(dsl::is_deleted.eq(false))
            .order(dsl::created_at.desc())
            .limit(10)
            .offset(0)
            .select(models::post::Post::as_select())
            .load::<Post>(&mut conn)
            .map_err(|e| e.to_string())?;

        Ok(posts)
    }

    async fn create_post(
        &self,
        user_id_value: i32,
        payload: CreatePostDto
    ) -> Result<Post, String> {
        let mut conn = self.database_pool.get().unwrap();

        let mut data = NewPost::from_dto(&payload);
        data.user_id = user_id_value;

        insert_into(posts::table)
            .values(data)
            .returning(Post::as_returning())
            .get_result(  &mut conn)
            .map_err(|e| e.to_string())
    }
}