use diesel::{prelude::{Insertable, Queryable}, Selectable};

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct User {
	pub id: i32,
	pub username: String,
	pub email: String,
  pub password: String,
  pub bio: Option<String>,
  pub profile_picture: Option<String>,
  pub background_picture: Option<String>,
  pub created_at: Option<chrono::NaiveDateTime>,
  pub updated_at: Option<chrono::NaiveDateTime>,
}

#[derive(Insertable)]
#[diesel(table_name = crate::schema::users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct NewUser {
  pub username: String,
  pub email: String,
  pub password: String
}