use diesel::{prelude::Queryable, Selectable};

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::post_medias)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[diesel(primary_key(post_id, media_id))]
#[diesel(belongs_to(crate::models::post::Post))]
#[diesel(belongs_to(crate::models::media::Media))]
pub struct PostMedia {
    pub post_id: i32,
    pub media_id: i32,
}