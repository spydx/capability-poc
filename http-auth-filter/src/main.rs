use actix_web::middleware::Logger;
use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
use actix_web_httpauth::middleware::HttpAuthentication;
use capabilities::{Capability, FilterConfig};

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
    let basepath = "http://localhost:8080/gnap/".to_string();
    let rs_ref = "e8a2968a-f183-45a3-b63d-4bbbd1dad276".to_string();
    let filter_config = FilterConfig::build(basepath, rs_ref);

    let bearer_auth = HttpAuthentication::bearer(capabilities::token_introspection);
    HttpServer::new(move || {
        App::new()
            .app_data(filter_config.clone())
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