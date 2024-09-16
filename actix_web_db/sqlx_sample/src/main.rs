use actix_web::{web, get, post, App, HttpResponse, HttpServer, Responder};
use actix_web::middleware::Logger;
use std::env;
use env_logger::Env;
use dotenv::dotenv;

use sqlx::sqlite::SqlitePool;
mod db;
use db::User;


pub struct AppState {
    db_pool: SqlitePool,
}

#[get("/")]
async fn get_users(data: web::Data<AppState>) -> impl Responder {
    let users = db::get_users(&data.db_pool).await;
    println!("{:?}", users);
    HttpResponse::Ok().json(users)
}

#[post("/add-user")]
async fn add_user(data: web::Data<AppState>, req: web::Json<User>) -> impl Responder {
    println!("{:?}", &req.id);
    println!("{:?}", &req.name);

    db::add_user(&data.db_pool, req.id, req.name.clone()).await;
    HttpResponse::Ok().body("OK")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // init db
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = SqlitePool::connect(&database_url).await.unwrap();

    env_logger::init_from_env(Env::default().default_filter_or("info"));

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(AppState { db_pool: pool.clone() }))
            .service(get_users)
            .service(add_user)
            .wrap(Logger::default())
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

