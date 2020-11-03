use crate::data_func::db_helpers::get_rdata;
use crate::errors::{AppError, AppErrorType};
use crate::Pool;

use actix_identity::Identity;
use actix_web::{get, web, web::Path, HttpResponse};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct GetParams {
    uuid: String,
}

#[get("/speculare/{uuid}")]
pub async fn index(
    id: Identity,
    params: Path<GetParams>,
    db: web::Data<Pool>,
) -> Result<HttpResponse, AppError> {
    // If the user is not identified, restrict access
    if !id.identity().is_some() {
        return Err(AppError {
            cause: None,
            message: Some("You're not allowed to access this resource".to_string()),
            error_type: AppErrorType::InvalidRequest,
        });
    }

    // Retrieve the uuid from the query
    let muuid = params.uuid.to_string();

    if log_enabled!(log::Level::Info) {
        info!("Route GET /speculare/{}", muuid);
    }

    // Return the data as form of JSON
    Ok(HttpResponse::Ok().json(get_rdata(muuid, db.get()?)?))
}
