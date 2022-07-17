use actix_web::{web, HttpResponse};
use sproot::apierrors::ApiError;
use sproot::models::BaseMetrics;
use sproot::models::CpuStats;
use sproot::models::MetricsPool;

use super::SpecificDated;

/// GET /api/cpustats
/// Return cpustats for a particular host
pub async fn cpustats(
    metrics: web::Data<MetricsPool>,
    info: web::Query<SpecificDated>,
) -> Result<HttpResponse, ApiError> {
    trace!("Route GET /api/cpustats : {:?}", info);

    let data = web::block(move || {
        CpuStats::get_dated(
            &mut metrics.pool.get()?,
            &info.uuid,
            info.min_date,
            info.max_date,
        )
    })
    .await??;

    Ok(HttpResponse::Ok().json(data))
}
