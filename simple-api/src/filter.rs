use actix_web;
use actix_web::dev::ServiceRequest;
use actix_web_httpauth::extractors::bearer::BearerAuth;

pub async fn token_introspection(
    req: ServiceRequest,
    _header: BearerAuth,
) -> Result<ServiceRequest, actix_web::Error> {
    println!("{}", _header.token());
    println!("{:#?}", req);
    Ok(req)
}