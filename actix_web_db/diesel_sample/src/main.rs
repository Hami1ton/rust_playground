use actix_web::{web, get, post, App, HttpResponse, HttpServer, Responder};
use actix_web::middleware::Logger;
use env_logger::Env;
use dotenv::dotenv;
use std::env;
use diesel::{prelude::*, r2d2};

mod db;
mod model;
mod schema;


type DbPool = r2d2::Pool<r2d2::ConnectionManager<SqliteConnection>>;
pub struct AppState {
    db_pool: DbPool,
}

#[get("/")]
async fn get_users(data: web::Data<AppState>) -> impl Responder {
    let mut conn = data.db_pool.get().unwrap();
    let users = db::get_users(&mut conn).await;

    HttpResponse::Ok().json(users)
}

#[post("/add-user")]
async fn add_user(data: web::Data<AppState>, req: web::Json<model::User>) -> impl Responder {
    println!("{:?}", &req.id);
    println!("{:?}", &req.name);

    let mut conn = data.db_pool.get().unwrap();
    db::add_user(&mut conn, req.id, req.name.clone()).await;

    HttpResponse::Ok().body("OK")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    // init db
    let pool = initialize_db_pool();

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

fn initialize_db_pool() -> DbPool {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let manager = r2d2::ConnectionManager::<SqliteConnection>::new(database_url);

    return r2d2::Pool::builder()
        .build(manager)
        .expect("error");
}