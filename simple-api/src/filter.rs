use actix_web::dev::ServiceRequest;
use actix_web::Error;
use actix_web_httpauth::extractors::bearer::BearerAuth;

pub async fn token_introspection(
    req: ServiceRequest,
    _header: BearerAuth,
) -> Result<ServiceRequest, Error> {
    println!("{}", _header.token());
    println!("{:#?}", req);
    Ok(req)
}
