use actix_web::{get, App, HttpResponse, HttpServer, Responder};

use actix_web::middleware::Logger;
use env_logger::Env;
use sqlx::sqlite::SqlitePool;
use std::env;
use dotenv::dotenv;
use sqlx::Error;
use sqlx::Sqlite;
use sqlx::Pool;


#[derive(sqlx::FromRow, Debug)]
pub struct User {
    pub id: i64,
    pub name: String,
}

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    env_logger::init_from_env(Env::default().default_filter_or("info"));

    let pool = init_db();
    let users = sqlx::query_as::<_, User>("SELECT * FROM user").fetch_all(&pool.await.unwrap()).await;

    if let Ok(user) = users {
        println!("{:?}", user);
    }

    HttpServer::new(|| {
        App::new()
            .service(hello)
            .wrap(Logger::default())
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

async fn init_db() -> Result<Pool<Sqlite>, Error> {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = SqlitePool::connect(&database_url).await.unwrap();
    
    return Ok(pool);
}

