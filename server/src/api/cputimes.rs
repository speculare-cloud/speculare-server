use sproot::errors::{AppError, AppErrorType};
use sproot::models::CpuTimes;
use sproot::Pool;

use super::PagedInfoSpecific;

use actix_web::{web, HttpResponse};

/// GET /api/cputimes
/// Return cputimes for a particular host
pub async fn cputimes(
    db: web::Data<Pool>,
    info: web::Query<PagedInfoSpecific>,
) -> Result<HttpResponse, AppError> {
    info!("Route GET /api/cputimes : {:?}", info);

    let uuid = info.uuid.to_owned();
    // If min_date and max_date are specified, it's a dated request, otherwise, normal
    // use web::block to offload blocking Diesel code without blocking server thread
    if info.min_date.is_some() && info.max_date.is_some() {
        let data = web::block(move || {
            CpuTimes::get_data_dated(
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
            let data =
                web::block(move || CpuTimes::get_data(&db.get()?, &uuid, size, page)).await??;
            // Return the data as form of JSON
            Ok(HttpResponse::Ok().json(data))
        }
    }
}
