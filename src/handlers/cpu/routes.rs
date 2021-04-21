use crate::errors::{AppError, AppErrorType};
use crate::models::{CpuStats, LoadAvg};
use crate::Pool;

use crate::handlers::PagedInfoSpecific;

use actix_web::{web, HttpResponse};

/// GET /api/cpustats
/// Return cpustats for a particular host
pub async fn cpustats(
    db: web::Data<Pool>,
    info: web::Query<PagedInfoSpecific>,
) -> Result<HttpResponse, AppError> {
    info!("Route GET /api/cpustats : {:?}", info);

    let uuid = info.uuid.to_owned();
    // If size is over 500 or less than 30, return error
    let size = info.size.unwrap_or(100);
    let page = info.page.unwrap_or(0);
    if !(30..=500).contains(&size) {
        Err(AppError {
            message: Some("The size parameters must be 30 < size <= 500".to_string()),
            cause: None,
            error_type: AppErrorType::InvalidRequest,
        })
    } else {
        // use web::block to offload blocking Diesel code without blocking server thread
        let data = web::block(move || CpuStats::get_data(&db.get()?, &uuid, size, page)).await?;
        // Return the data as form of JSON
        Ok(HttpResponse::Ok().json(data))
    }
}

/// GET /api/load_avg
/// Return load_avg for a particular host
pub async fn load_avg(
    db: web::Data<Pool>,
    info: web::Query<PagedInfoSpecific>,
) -> Result<HttpResponse, AppError> {
    info!("Route GET /api/load_avg : {:?}", info);

    let uuid = info.uuid.to_owned();
    // If size is over 500 or less than 30, return error
    let size = info.size.unwrap_or(100);
    let page = info.page.unwrap_or(0);
    if !(30..=500).contains(&size) {
        Err(AppError {
            message: Some("The size parameters must be 30 < size <= 500".to_string()),
            cause: None,
            error_type: AppErrorType::InvalidRequest,
        })
    } else {
        let data = web::block(move || {
            LoadAvg::get_data(&db.get()?, &uuid, size, page, info.min_date, info.max_date)
        })
        .await?;
        // Return the data as form of JSON
        Ok(HttpResponse::Ok().json(data))
    }
}
