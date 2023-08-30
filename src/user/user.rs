use serde::{Deserialize, Serialize};
use sqlx::mysql::MySqlPool;

#[derive(Debug, sqlx::FromRow, Serialize, Deserialize)]
pub struct User {
    pub id: u32,
    pub name: String,
}

pub mod user {}

pub async fn get_user(id: u32, pool: MySqlPool) -> Result<User, sqlx::Error> {
    let user = sqlx::query_as::<_, User>("SELECT id, name FROM users WHERE id = $1")
        .bind(id)
        .fetch_one(&pool)
        .await?;
    Ok(user)
}
