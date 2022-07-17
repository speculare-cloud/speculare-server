use actix_web::{web, HttpResponse};
use sproot::apierrors::ApiError;
use sproot::models::BaseMetrics;
use sproot::models::ExtMetrics;
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

/// GET /api/ioblocks_count
/// Return ioblocks_count for a particular host
pub async fn ioblocks_count(
    metrics: web::Data<MetricsPool>,
    info: web::Query<SpecificDated>,
) -> Result<HttpResponse, ApiError> {
    trace!("Route GET /api/ioblocks_count : {:?}", info);

    let data = web::block(move || {
        IoBlock::count_unique(
            &mut metrics.pool.get()?,
            &info.uuid,
            info.min_date,
            info.max_date,
        )
    })
    .await??;

    Ok(HttpResponse::Ok().body(data.to_string()))
}
