use crate::api::SpecificPaged;

use actix_web::{web, HttpResponse};
use sproot::apierrors::ApiError;
use sproot::models::Alerts;
use sproot::models::MetricsPool;
use sproot::models::Specific;

/// GET /api/alerts
/// Return all alerts
pub async fn alerts_list(
    metrics: web::Data<MetricsPool>,
    info: web::Query<SpecificPaged>,
) -> Result<HttpResponse, ApiError> {
    info!("Route GET /api/alerts");

    let (size, page) = info.get_size_page()?;

    let data =
        web::block(move || Alerts::get_list_host(&mut metrics.pool.get()?, &info.uuid, size, page))
            .await??;

    Ok(HttpResponse::Ok().json(data))
}

/// GET /api/alerts
/// Create a new alert for the specific host
pub async fn alerts_create(
    _metrics: web::Data<MetricsPool>,
    _info: web::Query<Specific>,
) -> Result<HttpResponse, ApiError> {
    info!("Route POST /api/alerts");
    todo!()
}

/// GET /api/alerts
/// Update a specific alert
pub async fn alerts_update(
    _metrics: web::Data<MetricsPool>,
    _info: web::Query<Specific>,
) -> Result<HttpResponse, ApiError> {
    info!("Route PATCH /api/alerts");
    todo!()
}

/// GET /api/alerts
/// Delete a specific alert
pub async fn alerts_delete(
    _metrics: web::Data<MetricsPool>,
    _info: web::Query<Specific>,
) -> Result<HttpResponse, ApiError> {
    info!("Route DELETE /api/alerts");
    todo!()
}
