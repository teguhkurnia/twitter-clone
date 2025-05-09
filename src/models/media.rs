use diesel::{prelude::Queryable, Selectable};

#[derive(diesel_derive_enum::DbEnum, Debug)]
#[ExistingTypePath = "crate::schema::sql_types::MediaType"]
pub enum MediaType {
    Image,
    Video
}

#[derive(diesel_derive_enum::DbEnum, Debug)]
#[ExistingTypePath = "crate::schema::sql_types::ModerationStatus"]
pub enum ModerationStatus {
    Pending,
    Approved,
    Rejected
}

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::medias)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Media {
    pub id: i32,
    pub user_id: i32,
    pub name: String,
    pub media_type: MediaType,
    pub mime_type: String,
    pub extension: String,
    pub width: i32,
    pub height: i32,
    pub duration: Option<i32>,
    pub size: i64,
    pub path: String,
    pub moderation_status: ModerationStatus,
    pub moderation_reason: Option<String>,
    pub is_deleted: Option<bool>,
    pub created_at: Option<chrono::NaiveDateTime>,
    pub updated_at: Option<chrono::NaiveDateTime>,
}