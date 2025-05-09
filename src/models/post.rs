use std::str::FromStr;

use diesel::{prelude::Queryable, Selectable};
use serde::{Deserialize, Serialize};

#[derive(diesel_derive_enum::DbEnum, Debug, Serialize, Deserialize)]
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

#[derive(Queryable, Selectable, Serialize, Deserialize)]
#[diesel(table_name = crate::schema::posts)]
#[diesel(check_for_backend(diesel::pg::Pg))]
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
