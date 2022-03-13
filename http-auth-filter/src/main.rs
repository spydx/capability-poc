use actix_web::middleware::Logger;
use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
use actix_web_httpauth::middleware::HttpAuthentication;
use capabilities::Capability;

/*
    1. Needs to be a Beaer token.
    2. Send the Bearer token to AS
    3. Validate Access Against Request

*/

#[actix_web::main]
async fn main() -> Result<(), std::io::Error> {
    println!("Hello, world!");

    let root = "/api";
    let binding = "0.0.0.0:8080";

    env_logger::init_from_env(env_logger::Env::new().default_filter_or("debug"));

    let bearer_auth = HttpAuthentication::bearer(capabilities::token_introspection);
    HttpServer::new(move || {
        App::new()
            .wrap(bearer_auth.clone())
            .wrap(Logger::default())
            .service(web::scope(&root).service(hello))
    })
    .bind(binding)?
    .run()
    .await?;

    Ok(())
}

/*
pub async fn hello(cap: ReqData<Vec<Capability>>) -> impl Responder {
    let cap = cap.into_inner();
*/
#[get("/")]
//pub async fn hello(cap: ReqData<Capability>) -> impl Responder {
pub async fn hello(cap: Capability) -> impl Responder {
    println!("Handler: {:#?}", cap);
    HttpResponse::Ok().body("hello from server")
}