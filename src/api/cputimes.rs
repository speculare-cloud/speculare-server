use super::SpecificDated;

use actix_web::{web, HttpResponse};
use sproot::errors::AppError;
use sproot::models::CpuTimes;
use sproot::models::MetricsPool;

/// GET /api/cputimes
/// Return cputimes for a particular host
pub async fn cputimes(
    metrics: web::Data<MetricsPool>,
    info: web::Query<SpecificDated>,
) -> Result<HttpResponse, AppError> {
    trace!("Route GET /api/cputimes : {:?}", info);

    let data = web::block(move || {
        CpuTimes::get_data_dated(
            &metrics.pool.get()?,
            &info.uuid,
            info.min_date,
            info.max_date,
        )
    })
    .await??;

    Ok(HttpResponse::Ok().json(data))
}
