use actix_web::{web, HttpResponse};
use sproot::apierrors::ApiError;
use sproot::models::BaseMetrics;
use sproot::models::IoNet;
use sproot::models::MetricsPool;

use super::SpecificDated;

/// GET /api/ionets
/// Return ionets for a particular host
pub async fn ionets(
    metrics: web::Data<MetricsPool>,
    info: web::Query<SpecificDated>,
) -> Result<HttpResponse, ApiError> {
    trace!("Route GET /api/ionets : {:?}", info);

    let data = web::block(move || {
        IoNet::get_dated(
            &mut metrics.pool.get()?,
            &info.uuid,
            info.min_date,
            info.max_date,
        )
    })
    .await??;

    Ok(HttpResponse::Ok().json(data))
}
