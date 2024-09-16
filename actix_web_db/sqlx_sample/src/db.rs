extern crate serde_json;

use sqlx::sqlite::SqlitePool;
use serde::{Deserialize, Serialize};


#[derive(sqlx::FromRow, Debug, Deserialize, Serialize)]
pub struct User {
    pub id: i64,
    pub name: String,
}

pub async fn get_users(db_pool: &SqlitePool) -> Vec<User>{
    let users = sqlx::query_as::<_, User>("SELECT * FROM user").fetch_all(db_pool).await;
    return users.expect("error");
}

pub async fn add_user(db_pool: &SqlitePool, id: i64, name: String) {
    let _ = sqlx::query(
        "INSERT INTO user (id, name) VALUES (?, ?)",
    )
    .bind(id)
    .bind(name)
    .execute(db_pool)
    .await;
}

