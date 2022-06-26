use super::SpecificDated;

use actix_web::{web, HttpResponse};
use sproot::apierrors::ApiError;
use sproot::models::MetricsPool;
use sproot::models::Swap;

/// GET /api/swap
/// Return swap for a particular host
pub async fn swap(
    metrics: web::Data<MetricsPool>,
    info: web::Query<SpecificDated>,
) -> Result<HttpResponse, ApiError> {
    trace!("Route GET /api/swap : {:?}", info);

    let data = web::block(move || {
        Swap::get_data_dated(
            &mut metrics.pool.get()?,
            &info.uuid,
            info.min_date,
            info.max_date,
        )
    })
    .await??;

    Ok(HttpResponse::Ok().json(data))
}
