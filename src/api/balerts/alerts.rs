use actix_web::{web, HttpResponse};
use sproot::apierrors::ApiError;
use sproot::models::Alerts;
use sproot::models::AlertsDTO;
use sproot::models::BaseCrud;
use sproot::models::DtoBase;
use sproot::models::ExtCrud;
use sproot::models::MetricsPool;
use sproot::models::Specific;

use crate::api::SpecificAlert;
use crate::api::SpecificPaged;

/// GET /api/alerts
/// Return all alerts
pub async fn alerts_list(
    metrics: web::Data<MetricsPool>,
    info: web::Query<SpecificPaged>,
) -> Result<HttpResponse, ApiError> {
    info!("Route GET /api/alerts");

    let (size, page) = info.get_size_page()?;

    let data =
        web::block(move || Alerts::get(&mut metrics.pool.get()?, &info.uuid, size, page)).await??;

    Ok(HttpResponse::Ok().json(data))
}

/// POST /api/alerts
/// Create a new alert for the specific host
pub async fn alerts_create(
    _metrics: web::Data<MetricsPool>,
    _info: web::Query<Specific>,
) -> Result<HttpResponse, ApiError> {
    info!("Route POST /api/alerts");
    todo!()
}

/// PATCH /api/alerts
/// Update a specific alert
pub async fn alerts_update(
    metrics: web::Data<MetricsPool>,
    info: web::Query<SpecificAlert>,
    item: web::Json<AlertsDTO>,
) -> Result<HttpResponse, ApiError> {
    info!("Route PATCH /api/alerts");

    let data = web::block(move || Alerts::update_and_get(&mut metrics.pool.get()?, info.id, &item))
        .await??;

    Ok(HttpResponse::Ok().json(data))
}

/// DELETE /api/alerts
/// Delete a specific alert
pub async fn alerts_delete(
    metrics: web::Data<MetricsPool>,
    info: web::Query<SpecificAlert>,
) -> Result<HttpResponse, ApiError> {
    info!("Route DELETE /api/alerts");

    let data = web::block(move || Alerts::delete(&mut metrics.pool.get()?, info.id)).await??;

    Ok(HttpResponse::Ok().body(data.to_string()))
}

/// GET /api/alerts/count
/// Return a count of incidents within size limit (or 100 if undefined)
pub async fn alerts_count(
    metrics: web::Data<MetricsPool>,
    info: web::Query<SpecificPaged>,
) -> Result<HttpResponse, ApiError> {
    info!("Route GET /api/alerts/count");

    let (size, _) = info.get_size_page()?;

    let data =
        web::block(move || Alerts::count(&mut metrics.pool.get()?, &info.uuid, size)).await??;

    Ok(HttpResponse::Ok().json(data))
}
