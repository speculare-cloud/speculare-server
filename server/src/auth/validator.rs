use actix_web::{dev::ServiceRequest, Error};
use actix_web_httpauth::extractors::bearer::BearerAuth;

pub async fn validator(
    req: ServiceRequest,
    _credentials: BearerAuth,
) -> Result<ServiceRequest, Error> {
    info!("Validator: validating the request");
    // TODO - Validate the token using the Auth Server's DB
    Ok(req)
}
