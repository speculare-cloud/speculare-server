use actix_web::{web, HttpResponse};
use sproot::apierrors::ApiError;
use sproot::models::BaseMetrics;
use sproot::models::LoadAvg;
use sproot::models::MetricsPool;

use super::SpecificDated;

/// GET /api/load_avg
/// Return load_avg for a particular host
pub async fn loadavg(
    metrics: web::Data<MetricsPool>,
    info: web::Query<SpecificDated>,
) -> Result<HttpResponse, ApiError> {
    trace!("Route GET /api/loadavg : {:?}", info);

    let data = web::block(move || {
        LoadAvg::get_dated(
            &mut metrics.pool.get()?,
            &info.uuid,
            info.min_date,
            info.max_date,
        )
    })
    .await??;

    Ok(HttpResponse::Ok().json(data))
}
