use crate::errors::{AppError, AppErrorType};
use crate::models::Disks;
use crate::Pool;

use crate::api::PagedInfoSpecific;

use actix_web::{web, HttpResponse};

/// GET /api/disks
/// Return disks for a particular host
pub async fn disks(
    db: web::Data<Pool>,
    info: web::Query<PagedInfoSpecific>,
) -> Result<HttpResponse, AppError> {
    info!("Route GET /api/disks : {:?}", info);

    let uuid = info.uuid.to_owned();
    // If size is over 500 or less than 0, return error
    let size = info.size.unwrap_or(100);
    let page = info.page.unwrap_or(0);
    if !(0..=500).contains(&size) {
        Err(AppError {
            message: Some("The size parameters must be 0 < size <= 500".to_string()),
            cause: None,
            error_type: AppErrorType::InvalidRequest,
        })
    } else {
        // use web::block to offload blocking Diesel code without blocking server thread
        let data = web::block(move || Disks::get_data(&db.get()?, &uuid, size, page)).await?;
        // Return the data as form of JSON
        Ok(HttpResponse::Ok().json(data))
    }
}

/// GET /api/disks_count
/// Return disks_count for a particular host
pub async fn disks_count(
    db: web::Data<Pool>,
    info: web::Query<PagedInfoSpecific>,
) -> Result<HttpResponse, AppError> {
    info!("Route GET /api/disks_count : {:?}", info);

    let uuid = info.uuid.to_owned();
    // If size is over 500 or less than 30, return error
    let size = info.size.unwrap_or(100);
    if !(30..=500).contains(&size) {
        Err(AppError {
            message: Some("The size parameters must be 30 < size <= 500".to_string()),
            cause: None,
            error_type: AppErrorType::InvalidRequest,
        })
    } else {
        // use web::block to offload blocking Diesel code without blocking server thread
        let data = web::block(move || Disks::count(&db.get()?, &uuid, size)).await?;
        // Return the data as form of JSON
        Ok(HttpResponse::Ok().body(data.to_string()))
    }
}
