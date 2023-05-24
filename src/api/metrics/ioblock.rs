use actix_web::{web, HttpResponse};
use sproot::apierrors::ApiError;
use sproot::models::BaseMetrics;
use sproot::models::IoBlock;
use sproot::models::MetricsPool;

use super::SpecificDated;

/// GET /api/ioblocks
/// Return ioblock for a particular host
pub async fn ioblocks(
    metrics: web::Data<MetricsPool>,
    info: web::Query<SpecificDated>,
) -> Result<HttpResponse, ApiError> {
    trace!("Route GET /api/ioblocks : {:?}", info);

    let data = web::block(move || {
        IoBlock::get_dated(
            &mut metrics.pool.get()?,
            &info.uuid,
            info.min_date,
            info.max_date,
        )
    })
    .await??;

    Ok(HttpResponse::Ok().json(data))
}
