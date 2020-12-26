use crate::errors::{AppError, AppErrorType};
use crate::models::{Host, HostList, HttpPostHost};
use crate::types::Pool;

use super::{HostUuid, PagedInfo};

use actix_web::{web, web::Path, HttpResponse};

/// GET /api/speculare
/// Return all host basic informations
pub async fn host_all(
    db: web::Data<Pool>,
    info: web::Query<PagedInfo>,
) -> Result<HttpResponse, AppError> {
    if log_enabled!(log::Level::Info) {
        info!("Route GET /speculare");
    }

    // If size is over 500 or less than 30, return error
    let size = info.size.unwrap_or(100);
    let page = info.page.unwrap_or(0);
    if !(30..=500).contains(&size) {
        Err(AppError {
            message: Some("The size parameters must be 30 < size < 500".to_string()),
            cause: None,
            error_type: AppErrorType::InvalidRequest,
        })
    } else {
        // use web::block to offload blocking Diesel code without blocking server thread
        let data = web::block(move || HostList::list(&db.get()?, size, page)).await?;
        // Return the data as form of JSON
        Ok(HttpResponse::Ok().json(data))
    }
}

/// GET /speculare/uuid
/// Return all details for a particular host
pub async fn host_info(
    params: Path<HostUuid>,
    db: web::Data<Pool>,
) -> Result<HttpResponse, AppError> {
    // Retrieve the uuid from the query
    let muuid = params.uuid.to_string();

    if log_enabled!(log::Level::Info) {
        info!("Route GET /speculare/{}", muuid);
    }

    // use web::block to offload blocking Diesel code without blocking server thread
    let data = web::block(move || Host::get(&db.get()?, &muuid)).await?;
    // Return the data as form of JSON
    Ok(HttpResponse::Ok().json(data))
}

/// POST /speculare
/// Save data from a host into the db under his uuid
pub async fn host_ingest(
    db: web::Data<Pool>,
    item: web::Json<HttpPostHost>,
) -> Result<HttpResponse, AppError> {
    if log_enabled!(log::Level::Info) {
        info!("Route POST /speculare : {:?}", item);
    }
    // make all insert taking advantage of web::block to offload the request thread
    web::block(move || Host::insert(&db.get()?, &item.into_inner())).await?;
    // Return a 200 status code as everything went well
    Ok(HttpResponse::Ok().finish())
}
