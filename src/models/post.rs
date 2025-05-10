use std::str::FromStr;

use diesel::{prelude::{AsChangeset, Insertable, Queryable}, Selectable};
use serde::{Deserialize, Serialize};

use super::dto::create_post_dto::CreatePostDto;

#[derive(diesel_derive_enum::DbEnum, Debug, Serialize, Deserialize, PartialEq, Clone)]
#[ExistingTypePath = "crate::schema::sql_types::PostTypeEnum"]
pub enum PostType {
    Post,
    Quote,
    Repost
}

impl FromStr for PostType {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "post" => Ok(PostType::Post),
            "quote" => Ok(PostType::Quote),
            "repost" => Ok(PostType::Repost),
            _ => Err(format!("Invalid post type: {}", s)),
        }
    }
}

#[derive(Queryable, Selectable, Serialize, Deserialize, Debug, PartialEq, Clone, AsChangeset)]
#[diesel(table_name = crate::schema::posts)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[diesel(belongs_to(Post, foreign_key = parent_id))]
pub struct Post {
    pub id: i32,
    pub user_id: i32,
    pub parent_id: Option<i32>,
    pub attached_post_id: Option<i32>,
    pub is_thread: bool,
    pub content: String,
    pub is_deleted: bool,
    pub is_edited: bool,
    pub is_pinned: bool,
    pub post_type: PostType,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
    pub deleted_at: Option<chrono::NaiveDateTime>,
  
}

#[derive(Insertable)]
#[diesel(table_name = crate::schema::posts)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct NewPost {
    pub user_id: i32,
    pub parent_id: Option<i32>,
    pub attached_post_id: Option<i32>,
    pub is_thread: bool,
    pub content: String,
    pub is_deleted: bool,
    pub is_edited: bool,
    pub is_pinned: bool,
    pub post_type: PostType,
}

impl NewPost {
    pub fn from_dto(dto: &CreatePostDto) -> Self {
        NewPost {
            user_id: 0,
            parent_id: dto.parent_id,
            attached_post_id: dto.attached_post_id,
            is_thread: dto.is_thread.unwrap_or(false),
            content: dto.content.clone(),
            is_deleted: false,
            is_edited: false,
            is_pinned: false,
            post_type: PostType::from_str(&dto.post_type).unwrap_or(PostType::Post),
        }
    }
}