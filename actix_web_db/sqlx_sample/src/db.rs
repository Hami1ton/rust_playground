extern crate serde_json;

use sqlx::Sqlite;
use sqlx::Pool;
use sqlx::sqlite::SqlitePool;
use std::env;
use serde::{Deserialize, Serialize};


static mut POOL_CACHE: Option<Pool<Sqlite>> = None;


#[derive(sqlx::FromRow, Debug, Deserialize, Serialize)]
pub struct User {
    pub id: i64,
    pub name: String,
}

pub async fn get_users() -> Vec<User>{
    unsafe {
        let users = sqlx::query_as::<_, User>("SELECT * FROM user").fetch_all(&POOL_CACHE.clone().unwrap()).await;
        return users.expect("error");
    }
}

pub async fn init_db() {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = SqlitePool::connect(&database_url).await.unwrap();
    
    unsafe { POOL_CACHE = Some(SqlitePool::connect(&database_url).await.unwrap()) };
}
