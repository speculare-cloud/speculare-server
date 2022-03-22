use crate::server::AppData;

use super::SpecificDated;

use actix_web::{web, HttpResponse};
use sproot::errors::AppError;
use sproot::models::Memory;

/// GET /api/memory
/// Return swap for a particular host
pub async fn memory(
    app_data: web::Data<AppData>,
    info: web::Query<SpecificDated>,
) -> Result<HttpResponse, AppError> {
    trace!("Route GET /api/memory : {:?}", info);

    let data = web::block(move || {
        Memory::get_data_dated(
            &app_data.metrics_db.get()?,
            &info.uuid,
            info.min_date,
            info.max_date,
        )
    })
    .await??;

    Ok(HttpResponse::Ok().json(data))
}
