use serde::{Deserialize, Serialize};
use sqlx::mysql::MySqlPool;

#[derive(Debug, sqlx::FromRow, Serialize, Deserialize, Clone)]
pub struct User {
    pub id: u32,
    pub name: String,
}

impl<'a> Default for User {
    fn default() -> User {
        User {
            id: 0,
            name: "".to_string(),
        }
    }
}

pub async fn get_user(id: u32, pool: MySqlPool) -> Result<User, sqlx::Error> {
    let user = sqlx::query_as::<_, User>("SELECT id, name FROM users WHERE id = ?")
        .bind(id)
        .fetch_one(&pool)
        .await?;
    Ok(user)
}
