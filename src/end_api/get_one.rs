use crate::data::db_helpers::get_data_from;
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

    // use web::block to offload blocking Diesel code without blocking server thread
    let data = web::block(move || get_data_from(muuid, db.get()?)).await?;
    // Return the data as form of JSON
    Ok(HttpResponse::Ok().json(data))
}
