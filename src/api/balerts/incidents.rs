use actix_web::{web, HttpResponse};
use sproot::apierrors::ApiError;
use sproot::models::Incidents;
use sproot::models::MetricsPool;
use sproot::models::{BaseCrud, ExtCrud};

use crate::api::SpecificPaged;

/// GET /api/incidents
/// Return all incidents
pub async fn incidents_list(
    metrics: web::Data<MetricsPool>,
    info: web::Query<SpecificPaged>,
) -> Result<HttpResponse, ApiError> {
    info!("Route GET /api/incidents");

    let (size, page) = info.get_size_page()?;

    let data = web::block(move || Incidents::get(&mut metrics.pool.get()?, &info.uuid, size, page))
        .await??;

    Ok(HttpResponse::Ok().json(data))
}

/// GET /api/incidents_count
/// Return a count of incidents within size limit (or 100 if undefined)
pub async fn incidents_count(
    metrics: web::Data<MetricsPool>,
    info: web::Query<SpecificPaged>,
) -> Result<HttpResponse, ApiError> {
    info!("Route GET /api/incidents_count");

    let (size, _) = info.get_size_page()?;

    let data =
        web::block(move || Incidents::count(&mut metrics.pool.get()?, &info.uuid, size)).await??;

    Ok(HttpResponse::Ok().body(data.to_string()))
}
