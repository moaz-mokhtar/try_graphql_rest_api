use crate::{db::DbPool, entity::User};
use diesel::prelude::*;

type DbError = Box<dyn std::error::Error + Send + Sync>;

impl User {
    pub async fn get_all(pool: &DbPool) -> Result<Vec<User>, DbError> {
        use crate::schema::users::dsl::users;

        let connection = pool.get()?;
        let _users = users.load::<User>(&connection)?;

        Ok(_users)
    }
}
