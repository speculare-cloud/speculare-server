use crate::errors::AppError;
use crate::models::Swap;
use crate::Pool;

use crate::api::PagedInfoSpecific;

use actix_web::{web, HttpResponse};

/// GET /api/swap
/// Return swap for a particular host
pub async fn swap(
    db: web::Data<Pool>,
    info: web::Query<PagedInfoSpecific>,
) -> Result<HttpResponse, AppError> {
    info!("Route GET /api/swap : {:?}", info);

    let uuid = info.uuid.to_owned();
    // If size is over 500 or less than 30, return error
    let size = info.size.unwrap_or(100);
    let page = info.page.unwrap_or(0);
    // If min_date and max_date are specified, it's a dated request, otherwise, normal
    // use web::block to offload blocking Diesel code without blocking server thread
    if info.min_date.is_some() && info.max_date.is_some() {
        let data = web::block(move || {
            Swap::get_data_dated(
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
        let data = web::block(move || Swap::get_data(&db.get()?, &uuid, size, page)).await?;
        // Return the data as form of JSON
        Ok(HttpResponse::Ok().json(data))
    }
}
