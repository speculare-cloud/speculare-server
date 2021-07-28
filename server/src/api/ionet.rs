use sproot::errors::{AppError, AppErrorType};
use sproot::models::IoNet;
use sproot::Pool;

use super::PagedInfoSpecific;

use actix_web::{http, web, HttpResponse};

/// GET /api/ionets
/// Return ionets for a particular host
pub async fn ionets(
    db: web::Data<Pool>,
    info: web::Query<PagedInfoSpecific>,
) -> Result<HttpResponse, AppError> {
    info!("Route GET /api/ionets : {:?}", info);

    let uuid = info.uuid.to_owned();
    // If min_date and max_date are specified, it's a dated request, otherwise, normal
    // use web::block to offload blocking Diesel code without blocking server thread
    if info.min_date.is_some() && info.max_date.is_some() {
        let data = web::block(move || {
            IoNet::get_data_dated(
                &db.get()?,
                &uuid,
                info.min_date.unwrap(),
                info.max_date.unwrap(),
            )
        })
        .await??;
        // Return the data as form of JSON
        Ok(HttpResponse::Ok().json(data))
    } else {
        let size = info.size.unwrap_or(100);
        let page = info.page.unwrap_or(0);
        // Define the max size someone can request
        if !(30..=5000).contains(&size) {
            Err(AppError {
                message: Some("The size parameters must be 30 < size <= 5000".to_string()),
                cause: None,
                error_type: AppErrorType::InvalidRequest,
            })
        } else {
            let data = web::block(move || IoNet::get_data(&db.get()?, &uuid, size, page)).await??;
            // Return the data as form of JSON
            Ok(HttpResponse::Ok().json(data))
        }
    }
}

/// GET /api/ionets_count
/// Return ionets_count for a particular host
pub async fn ionets_count(
    db: web::Data<Pool>,
    info: web::Query<PagedInfoSpecific>,
) -> Result<HttpResponse, AppError> {
    info!("Route GET /api/ionets_count : {:?}", info);

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
        let data = web::block(move || IoNet::count(&db.get()?, &uuid, size)).await??;
        // Return the data as form of JSON
        Ok(HttpResponse::Ok()
            .append_header((http::header::CONTENT_TYPE, "text/plain"))
            .body(data.to_string()))
    }
}
