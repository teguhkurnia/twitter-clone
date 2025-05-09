// @generated automatically by Diesel CLI.

pub mod sql_types {
    #[derive(diesel::query_builder::QueryId, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "media_type"))]
    pub struct MediaType;

    #[derive(diesel::query_builder::QueryId, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "moderation_status"))]
    pub struct ModerationStatus;

    #[derive(diesel::query_builder::QueryId, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "post_type_enum"))]
    pub struct PostTypeEnum;
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::MediaType;
    use super::sql_types::ModerationStatus;

    medias (id) {
        id -> Int4,
        user_id -> Int4,
        #[max_length = 255]
        name -> Varchar,
        media_type -> MediaType,
        #[max_length = 255]
        mime_type -> Varchar,
        #[max_length = 10]
        extension -> Varchar,
        width -> Int4,
        height -> Int4,
        duration -> Nullable<Int4>,
        size -> Int8,
        #[max_length = 255]
        path -> Varchar,
        moderation_status -> ModerationStatus,
        #[max_length = 255]
        moderation_reason -> Nullable<Varchar>,
        is_deleted -> Nullable<Bool>,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
        deleted_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    post_medias (post_id, media_id) {
        post_id -> Int4,
        media_id -> Int4,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::PostTypeEnum;

    posts (id) {
        id -> Int4,
        user_id -> Int4,
        parent_id -> Nullable<Int4>,
        attached_post_id -> Nullable<Int4>,
        is_thread -> Bool,
        #[max_length = 5000]
        content -> Varchar,
        is_deleted -> Bool,
        is_edited -> Bool,
        is_pinned -> Bool,
        post_type -> PostTypeEnum,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        deleted_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    users (id) {
        id -> Int4,
        #[max_length = 50]
        username -> Varchar,
        #[max_length = 255]
        email -> Varchar,
        #[max_length = 255]
        password -> Varchar,
        #[max_length = 255]
        bio -> Nullable<Varchar>,
        #[max_length = 255]
        profile_picture -> Nullable<Varchar>,
        #[max_length = 255]
        background_picture -> Nullable<Varchar>,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
    }
}

diesel::joinable!(medias -> users (user_id));
diesel::joinable!(post_medias -> medias (media_id));
diesel::joinable!(post_medias -> posts (post_id));
diesel::joinable!(posts -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(
    medias,
    post_medias,
    posts,
    users,
);
