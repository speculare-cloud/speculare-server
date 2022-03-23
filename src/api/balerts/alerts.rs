use crate::api::SpecificPaged;

use actix_web::{web, HttpResponse};
use sproot::errors::AppError;
use sproot::models::Alerts;
use sproot::models::MetricsPool;

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
