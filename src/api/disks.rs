use actix_web::{web, HttpResponse};
use sproot::apierrors::ApiError;
use sproot::models::BaseMetrics;
use sproot::models::Disk;
use sproot::models::ExtMetrics;
use sproot::models::MetricsPool;

use super::SpecificDated;

/// GET /api/disks
/// Return disks for a particular host
pub async fn disks(
    metrics: web::Data<MetricsPool>,
    info: web::Query<SpecificDated>,
) -> Result<HttpResponse, ApiError> {
    trace!("Route GET /api/disks : {:?}", info);

    let data = web::block(move || {
        Disk::get_dated(
            &mut metrics.pool.get()?,
            &info.uuid,
            info.min_date,
            info.max_date,
        )
    })
    .await??;

    Ok(HttpResponse::Ok().json(data))
}

/// GET /api/disks_count
/// Return disks_count for a particular host
pub async fn disks_count(
    metrics: web::Data<MetricsPool>,
    info: web::Query<SpecificDated>,
) -> Result<HttpResponse, ApiError> {
    trace!("Route GET /api/disks_count : {:?}", info);

    let data = web::block(move || {
        Disk::count_unique(
            &mut metrics.pool.get()?,
            &info.uuid,
            info.min_date,
            info.max_date,
        )
    })
    .await??;

    Ok(HttpResponse::Ok().body(data.to_string()))
}
