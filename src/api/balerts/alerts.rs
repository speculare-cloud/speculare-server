use crate::api::SpecificPaged;

use actix_web::{web, HttpResponse};
use sproot::errors::AppError;
use sproot::models::Alerts;
use sproot::models::MetricsPool;
#[cfg(feature = "auth")]
use sproot::models::Specific;

/// GET /api/alerts
/// Return all alerts
pub async fn alerts_list(
    metrics: web::Data<MetricsPool>,
    info: web::Query<SpecificPaged>,
) -> Result<HttpResponse, AppError> {
    info!("Route GET /api/alerts");

    let (size, page) = info.get_size_page()?;

    let data =
        web::block(move || Alerts::get_list_host(&metrics.pool.get()?, &info.uuid, size, page))
            .await??;

    Ok(HttpResponse::Ok().json(data))
}

/// GET /api/alerts
/// Create a new alert for the specific host
#[cfg(feature = "auth")]
pub async fn alerts_create(
    _metrics: web::Data<MetricsPool>,
    _info: web::Query<Specific>,
) -> Result<HttpResponse, AppError> {
    info!("Route POST /api/alerts");

    Ok(HttpResponse::Ok().finish())
}

/// GET /api/alerts
/// Update a specific alert
#[cfg(feature = "auth")]
pub async fn alerts_update(
    _metrics: web::Data<MetricsPool>,
    _info: web::Query<Specific>,
) -> Result<HttpResponse, AppError> {
    info!("Route PATCH /api/alerts");

    Ok(HttpResponse::Ok().finish())
}

/// GET /api/alerts
/// Delete a specific alert
#[cfg(feature = "auth")]
pub async fn alerts_delete(
    _metrics: web::Data<MetricsPool>,
    _info: web::Query<Specific>,
) -> Result<HttpResponse, AppError> {
    info!("Route DELETE /api/alerts");

    Ok(HttpResponse::Ok().finish())
}
