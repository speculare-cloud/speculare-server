use crate::api::SpecificPaged;

use actix_web::{web, HttpResponse};
use sproot::errors::AppError;
use sproot::models::Incidents;
use sproot::models::MetricsPool;

/// GET /api/incidents
/// Return all incidents
pub async fn incidents_list(
    metrics: web::Data<MetricsPool>,
    info: web::Query<SpecificPaged>,
) -> Result<HttpResponse, AppError> {
    info!("Route GET /api/incidents");

    let (size, page) = info.get_size_page()?;

    let data =
        web::block(move || Incidents::get_list_host(&metrics.pool.get()?, &info.uuid, size, page))
            .await??;

    Ok(HttpResponse::Ok().json(data))
}
