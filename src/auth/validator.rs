use crate::server::AppData;

use actix_web::{dev::ServiceRequest, error, Error};
use actix_web_httpauth::extractors::bearer::BearerAuth;
use sproot::models::ApiKey;

pub async fn validator(
    req: ServiceRequest,
    credentials: BearerAuth,
) -> Result<ServiceRequest, Error> {
    trace!("Validator: validating the request");
    // Get the app_data holding the database's pool reference
    let app_data = match req.app_data::<AppData>() {
        Some(data) => data,
        None => return Err(error::ErrorInternalServerError("No AppData")),
    };

    // Get a auth_db connection from the pool
    let conn = match app_data.metrics_db.get() {
        Ok(conn) => conn,
        Err(e) => return Err(error::ErrorInternalServerError(e)),
    };

    // Get the api_key from the database
    let api_key = match ApiKey::get_entry(&conn, credentials.token()) {
        Ok(key) => key,
        Err(e) => return Err(error::ErrorBadRequest(e)),
    };

    // Get the targeted host uuid
    let host_uuid = match req.headers().get("SP-UUID") {
        Some(uuid) => uuid,
        None => return Err(error::ErrorBadRequest("The host does not match")),
    };

    // If that does not match, wrong key...
    if &api_key.host_uuid != host_uuid {
        return Err(error::ErrorUnauthorized("Nope"));
    }

    Ok(req)
}
