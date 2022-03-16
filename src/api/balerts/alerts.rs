use crate::api::PagedInfo;
use crate::server::AppData;

use actix_web::{web, HttpResponse};
use sproot::errors::AppError;
use sproot::models::Alerts;

/// GET /api/alerts
/// Return all alerts
pub async fn alerts_list(
    app_data: web::Data<AppData>,
    info: web::Query<PagedInfo>,
) -> Result<HttpResponse, AppError> {
    info!("Route GET /api/alerts");

    let (size, page) = info.get_size_page()?;
    let data = web::block(move || {
        Alerts::get_list_host(&app_data.metrics_db.get()?, &info.uuid, size, page)
    })
    .await??;
    Ok(HttpResponse::Ok().json(data))
}
