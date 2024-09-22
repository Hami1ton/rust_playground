use actix_web::{get, App, HttpResponse, HttpServer, Responder};
use actix_web::middleware::Logger;
use env_logger::Env;
use dotenv::dotenv;
use std::env;
use diesel::{prelude::*, r2d2};

mod db;
mod model;
mod schema;


#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("OK")
}


#[actix_web::main]
async fn main() {
    dotenv().ok();
    // init db
    let pool = initialize_db_pool();
    let mut conn = pool.get().unwrap();
    db::get_users(&mut conn).await;
}


// #[actix_web::main]
// async fn main() -> std::io::Result<()> {
//     dotenv().ok();
//     // init db
//     let pool = initialize_db_pool();

//     env_logger::init_from_env(Env::default().default_filter_or("info"));

//     HttpServer::new(|| {
//         App::new()
//             .service(hello)
//             .wrap(Logger::default())
//     })
//     .bind(("127.0.0.1", 8080))?
//     .run()
//     .await
// }



type DbPool = r2d2::Pool<r2d2::ConnectionManager<SqliteConnection>>;

fn initialize_db_pool() -> DbPool {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let manager = r2d2::ConnectionManager::<SqliteConnection>::new(database_url);

    return r2d2::Pool::builder()
        .build(manager)
        .expect("error");
}