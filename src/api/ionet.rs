use super::{SpecificDated, SpecificPaged};

use actix_web::{web, HttpResponse};
use sproot::errors::AppError;
use sproot::models::IoNet;
use sproot::models::MetricsPool;

/// GET /api/ionets
/// Return ionets for a particular host
pub async fn ionets(
    metrics: web::Data<MetricsPool>,
    info: web::Query<SpecificDated>,
) -> Result<HttpResponse, AppError> {
    trace!("Route GET /api/ionets : {:?}", info);

    let data = web::block(move || {
        IoNet::get_data_dated(
            &mut metrics.pool.get()?,
            &info.uuid,
            info.min_date,
            info.max_date,
        )
    })
    .await??;

    Ok(HttpResponse::Ok().json(data))
}

/// GET /api/ionets_count
/// Return ionets_count for a particular host
pub async fn ionets_count(
    metrics: web::Data<MetricsPool>,
    info: web::Query<SpecificPaged>,
) -> Result<HttpResponse, AppError> {
    trace!("Route GET /api/ionets_count : {:?}", info);

    let data =
        web::block(move || IoNet::count(&mut metrics.pool.get()?, &info.uuid, info.get_size()?))
            .await??;

    Ok(HttpResponse::Ok().body(data.to_string()))
}
