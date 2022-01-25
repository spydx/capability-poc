use sqlx::SqlitePool;
use sqlx::sqlite::SqlitePoolOptions;

use actix_web::{App, HttpServer, HttpResponse, Responder, web};
use actix_web::middleware::Logger;
use actix_web::{get, post};

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    println!("Hello, world!");

    let root = "/api";
    let binding = "0.0.0.0:8081";

    env_logger::init_from_env(env_logger::Env::new().default_filter_or("debug"));

    let con_str = "sqlite:autorityserver.db".to_string();
    let database = SqlitePoolOptions::new()
        .connect(&con_str)
        .await
        .expect("Failed to get database connection");

    sqlx::migrate!("./migrations")
        .run(&database)
        .await
        .expect("Failed to run sql mig on database");

    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .service(
                web::scope(&root)
                    .service(login_user)
                    .service(get_claims)
                    .service(get_token)
            )
            .app_data(web::Data::new(database.clone()))
            
    })
    .bind(binding)?
    .run().await?;

    Ok(())

}


#[post("/login")]
pub async fn login_user(_pool: web::Data<SqlitePool>) -> impl Responder {
    HttpResponse::Ok().json("{ \"msg:\" \"not implemented\" }")
}

#[get("/claims")]
pub async fn get_claims(_pool: web::Data<SqlitePool>) -> impl Responder {
    HttpResponse::Ok().json("{ \"msg:\" \"not implemented\" }")
}

#[get("/token")]
pub async fn get_token() -> impl Responder {
    HttpResponse::Ok().json("{ \"msg:\" \"not implemented\" }")
}