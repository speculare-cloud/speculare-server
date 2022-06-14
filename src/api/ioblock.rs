use super::{SpecificDated, SpecificPaged};

use actix_web::{web, HttpResponse};
use sproot::errors::AppError;
use sproot::models::IoBlock;
use sproot::models::MetricsPool;

/// GET /api/ioblocks
/// Return ioblock for a particular host
pub async fn ioblocks(
    metrics: web::Data<MetricsPool>,
    info: web::Query<SpecificDated>,
) -> Result<HttpResponse, AppError> {
    trace!("Route GET /api/ioblocks : {:?}", info);

    let data = web::block(move || {
        IoBlock::get_data_dated(
            &metrics.pool.get()?,
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
    info: web::Query<SpecificPaged>,
) -> Result<HttpResponse, AppError> {
    trace!("Route GET /api/ioblocks_count : {:?}", info);

    let data =
        web::block(move || IoBlock::count(&metrics.pool.get()?, &info.uuid, info.get_size()?))
            .await??;

    Ok(HttpResponse::Ok().body(data.to_string()))
}
