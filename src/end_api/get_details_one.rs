use crate::data_func::db_helpers::get_rdata;
use crate::errors::AppError;
use crate::Pool;

use actix_web::{web, web::Path, HttpResponse};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct GetParams {
    uuid: String,
}

/// GET /speculare/uuid
/// Return all details for a particular host
pub async fn index(params: Path<GetParams>, db: web::Data<Pool>) -> Result<HttpResponse, AppError> {
    // Retrieve the uuid from the query
    let muuid = params.uuid.to_string();

    if log_enabled!(log::Level::Info) {
        info!("Route GET /speculare/{}", muuid);
    }

    // Return the data as form of JSON
    Ok(HttpResponse::Ok().json(get_rdata(muuid, db.get()?)?))
}
