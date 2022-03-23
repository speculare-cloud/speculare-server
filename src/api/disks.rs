use super::{SpecificDated, SpecificPaged};

use actix_web::{web, HttpResponse};
use sproot::errors::AppError;
use sproot::models::Disk;
use sproot::models::MetricsPool;

/// GET /api/disks
/// Return disks for a particular host
pub async fn disks(
    metrics: web::Data<MetricsPool>,
    info: web::Query<SpecificDated>,
) -> Result<HttpResponse, AppError> {
    trace!("Route GET /api/disks : {:?}", info);

    let data = web::block(move || {
        Disk::get_data_dated(
            &metrics.pool.get()?,
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
    info: web::Query<SpecificPaged>,
) -> Result<HttpResponse, AppError> {
    trace!("Route GET /api/disks_count : {:?}", info);

    let data = web::block(move || Disk::count(&metrics.pool.get()?, &info.uuid, info.get_size()?))
        .await??;

    Ok(HttpResponse::Ok().body(data.to_string()))
}
