use super::SpecificDated;

use actix_web::{web, HttpResponse};
use sproot::errors::AppError;
use sproot::models::Memory;
use sproot::models::MetricsPool;

/// GET /api/memory
/// Return swap for a particular host
pub async fn memory(
    metrics: web::Data<MetricsPool>,
    info: web::Query<SpecificDated>,
) -> Result<HttpResponse, AppError> {
    trace!("Route GET /api/memory : {:?}", info);

    let data = web::block(move || {
        Memory::get_data_dated(
            &mut metrics.pool.get()?,
            &info.uuid,
            info.min_date,
            info.max_date,
        )
    })
    .await??;

    Ok(HttpResponse::Ok().json(data))
}
