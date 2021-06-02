use crate::errors::{AppError, AppErrorType};
use crate::models::IoStats;
use crate::Pool;

use crate::api::PagedInfoSpecific;

use actix_web::{http, web, HttpResponse};

/// GET /api/iostats
/// Return iostats for a particular host
pub async fn iostats(
    db: web::Data<Pool>,
    info: web::Query<PagedInfoSpecific>,
) -> Result<HttpResponse, AppError> {
    info!("Route GET /api/iostats : {:?}", info);

    let uuid = info.uuid.to_owned();
    let size = info.size.unwrap_or(100);
    let page = info.page.unwrap_or(0);
    // If min_date and max_date are specified, it's a dated request, otherwise, normal
    // use web::block to offload blocking Diesel code without blocking server thread
    if info.min_date.is_some() && info.max_date.is_some() {
        let data = web::block(move || {
            IoStats::get_data_dated(
                &db.get()?,
                &uuid,
                size,
                info.min_date.unwrap(),
                info.max_date.unwrap(),
            )
        })
        .await?;
        // Return the data as form of JSON
        Ok(HttpResponse::Ok().json(data))
    } else {
        // Define the max size someone can request
        if !(30..=5000).contains(&size) {
            Err(AppError {
                message: Some("The size parameters must be 30 < size <= 5000".to_string()),
                cause: None,
                error_type: AppErrorType::InvalidRequest,
            })
        } else {
            let data = web::block(move || IoStats::get_data(&db.get()?, &uuid, size, page)).await?;
            // Return the data as form of JSON
            Ok(HttpResponse::Ok().json(data))
        }
    }
}

/// GET /api/iostats_count
/// Return iostats_count for a particular host
pub async fn iostats_count(
    db: web::Data<Pool>,
    info: web::Query<PagedInfoSpecific>,
) -> Result<HttpResponse, AppError> {
    info!("Route GET /api/iostats_count : {:?}", info);

    let uuid = info.uuid.to_owned();
    // If size is over 5000 or less than 30, return error
    let size = info.size.unwrap_or(100);
    if !(30..=5000).contains(&size) {
        Err(AppError {
            message: Some("The size parameters must be 30 < size <= 5000".to_string()),
            cause: None,
            error_type: AppErrorType::InvalidRequest,
        })
    } else {
        // use web::block to offload blocking Diesel code without blocking server thread
        let data = web::block(move || IoStats::count(&db.get()?, &uuid, size)).await?;
        // Return the data as form of JSON
        Ok(HttpResponse::Ok()
            .header(http::header::CONTENT_TYPE, "text/plain")
            .body(data.to_string()))
    }
}
