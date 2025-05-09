use diesel::{r2d2::ConnectionManager, BoolExpressionMethods, BoxableExpression, ExpressionMethods, PgConnection, QueryDsl, RunQueryDsl, SelectableHelper};
use r2d2::Pool;

use crate::{models::user, repositories::traits::user_repository::{UserIdentifier, UserRepositoryTrait}, schema::users::dsl};

pub struct UserRepository {
    database_pool: Pool<ConnectionManager<PgConnection>>,
}

impl UserRepository {
    pub fn new(database_pool: Pool<ConnectionManager<PgConnection>>) -> Self {
        UserRepository { database_pool }
    }
}

#[async_trait::async_trait]
impl UserRepositoryTrait for UserRepository {
    async fn find_user(&self, identifier: UserIdentifier) -> Option<user::User> {
        let mut conn = self.database_pool.get().unwrap();

        let filter: Box<dyn BoxableExpression<crate::schema::users::table, diesel::pg::Pg, SqlType = diesel::sql_types::Bool>> =
            match identifier {
                UserIdentifier::Username(username_val) => {
                    Box::new(dsl::username.eq(username_val))
                }
                UserIdentifier::Email(email_val) => {
                    Box::new(dsl::email.eq(email_val))
                }
                UserIdentifier::UsernameOrEmail { username, email } => {
                    Box::new(dsl::username.eq(username).or(dsl::email.eq(email)))
                }
            };

        match dsl::users
            .filter(filter) 
            .select(user::User::as_select()).first::<user::User>(&mut conn) {
                Ok(user) => Some(user),
                Err(_) => None,
            }
    }

    async fn create_user(&self, user: &user::NewUser) -> Result<user::User, String> {
        let mut conn = self.database_pool.get().unwrap();
        match diesel::insert_into(dsl::users)
            .values(user)
            .get_result::<user::User>(&mut conn)
        {
            Ok(user) => Ok(user),
            Err(err) => Err(err.to_string()),
        }
    }
}   