use crate::errors::{AppError, AppErrorType};
use crate::models::Memory;
use crate::Pool;

use crate::api::PagedInfoSpecific;

use actix_web::{web, HttpResponse};

/// GET /api/memory
/// Return swap for a particular host
pub async fn memory(
    db: web::Data<Pool>,
    info: web::Query<PagedInfoSpecific>,
) -> Result<HttpResponse, AppError> {
    info!("Route GET /api/memory : {:?}", info);

    let uuid = info.uuid.to_owned();
    // If size is over 500 or less than 30, return error
    let size = info.size.unwrap_or(100);
    let page = info.page.unwrap_or(0);
    if !(30..=50000).contains(&size) {
        Err(AppError {
            message: Some("The size parameters must be 30 < size <= 500".to_string()),
            cause: None,
            error_type: AppErrorType::InvalidRequest,
        })
    } else {
        // If min_date and max_date are specified, it's a dated request, otherwise, normal
        // use web::block to offload blocking Diesel code without blocking server thread
        let data = if info.min_date.is_some() && info.max_date.is_some() {
            web::block(move || {
                Memory::get_data_dated(
                    &db.get()?,
                    &uuid,
                    size,
                    page,
                    info.min_date.unwrap(),
                    info.max_date.unwrap(),
                )
            })
            .await?
        } else {
            web::block(move || Memory::get_data(&db.get()?, &uuid, size, page)).await?
        };
        // Return the data as form of JSON
        Ok(HttpResponse::Ok().json(data))
    }
}
