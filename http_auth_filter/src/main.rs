use actix_web::{App, HttpServer, HttpResponse, Responder, Error, web, get};
use actix_web::dev::ServiceRequest;
use actix_web_httpauth::extractors::bearer::BearerAuth;
use actix_web_httpauth::middleware::HttpAuthentication;
use actix_web::middleware::Logger;
use gnap_cli::introspect;
use log::debug;

/*
    1. Needs to be a Beaer token.
    2. Send the Bearer token to AS
    3. Validate Access Against Request

*/


const BASEPATH: &str = "http://localhost:8000/gnap";


async fn token_introspection(
    req: ServiceRequest,
    header: BearerAuth,
) -> Result<ServiceRequest, Error> {
    debug!("Token: {}", header.token());
    debug!("{:#?}", req);

    let token = header.token().to_string();
    let rs_ref = "e8a2968a-f183-45a3-b63d-4bbbd1dad276".to_string();
    let url = format!("{}", BASEPATH);
    
    let _valid = match introspect(token, rs_ref, url).await {
        Ok(ir) =>  {
            println!("{:#?}", ir);
            match ir.active {
                true => {
                    debug!("{:#?}", ir);
                    ir
                },
                false => 
                    return Err(actix_web::error::ErrorForbidden("body1"))
            }
        },
        Err(_) => {
            return Err(actix_web::error::ErrorForbidden("body2"))
        }
    };

    
    Ok(req)
}



#[actix_web::main]
async fn main() -> Result<(), std::io::Error> {
    println!("Hello, world!");

    let root = "/api";
    let binding = "0.0.0.0:8080";

    env_logger::init_from_env(env_logger::Env::new().default_filter_or("debug"));
    
    let bearer_auth = HttpAuthentication::bearer(token_introspection);
    HttpServer::new(move || {
        App::new()
            .wrap(bearer_auth.clone())
            .wrap(Logger::default())
            .service(
                web::scope(&root)
                    .service(hello)
            )
    })
    .bind(binding)?
    .run().await?;

    Ok(())
}


#[get("/")]
pub async fn hello() -> impl Responder {
    HttpResponse::Ok().body("hello from server")
}
