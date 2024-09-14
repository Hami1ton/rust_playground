use actix_web::{web, get, post, App, HttpResponse, HttpServer, Responder};
use actix_web::middleware::Logger;

use env_logger::Env;
use dotenv::dotenv;
mod db;
use db::User;

#[get("/")]
async fn get_users() -> impl Responder {
    let users = db::get_users().await;
    println!("{:?}", users);
    HttpResponse::Ok().json(users)
}

#[post("/add-user")]
async fn add_user(req: web::Json<User>) -> impl Responder {
    println!("{:?}", &req.id);
    println!("{:?}", &req.name);

    db::add_user(req.id, req.name.clone()).await;
    HttpResponse::Ok().body("Hello")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // init db
    dotenv().ok();
    db::init_db().await;

    env_logger::init_from_env(Env::default().default_filter_or("info"));

    HttpServer::new(|| {
        App::new()
            .service(get_users)
            .service(add_user)
            .wrap(Logger::default())
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

