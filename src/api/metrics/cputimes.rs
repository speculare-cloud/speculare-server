use actix_web::{web, HttpResponse};
use sproot::apierrors::ApiError;
use sproot::models::BaseMetrics;
use sproot::models::CpuTimes;
use sproot::models::MetricsPool;

use super::SpecificDated;

/// GET /api/cputimes
/// Return cputimes for a particular host
pub async fn cputimes(
    metrics: web::Data<MetricsPool>,
    info: web::Query<SpecificDated>,
) -> Result<HttpResponse, ApiError> {
    trace!("Route GET /api/cputimes : {:?}", info);

    let data = web::block(move || {
        CpuTimes::get_dated(
            &mut metrics.pool.get()?,
            &info.uuid,
            info.min_date,
            info.max_date,
        )
    })
    .await??;

    Ok(HttpResponse::Ok().json(data))
}
