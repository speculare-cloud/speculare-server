use sproot::errors::{AppError, AppErrorType};
use sproot::models::Memory;
use sproot::Pool;

use super::PagedInfoSpecific;

use actix_web::{web, HttpResponse};

/// GET /api/memory
/// Return swap for a particular host
pub async fn memory(
    db: web::Data<Pool>,
    info: web::Query<PagedInfoSpecific>,
) -> Result<HttpResponse, AppError> {
    info!("Route GET /api/memory : {:?}", info);

    let uuid = info.uuid.to_owned();
    let size = info.size.unwrap_or(100);
    let page = info.page.unwrap_or(0);
    // If min_date and max_date are specified, it's a dated request, otherwise, normal
    // use web::block to offload blocking Diesel code without blocking server thread
    if info.min_date.is_some() && info.max_date.is_some() {
        let data = web::block(move || {
            Memory::get_data_dated(
                &db.get()?,
                &uuid,
                size,
                info.min_date.unwrap(),
                info.max_date.unwrap(),
            )
        })
        .await??;
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
            let data =
                web::block(move || Memory::get_data(&db.get()?, &uuid, size, page)).await??;
            // Return the data as form of JSON
            Ok(HttpResponse::Ok().json(data))
        }
    }
}
