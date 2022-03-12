use crate::api::PagedInfo;
use crate::server::AppData;

use actix_web::{web, HttpResponse};
use sproot::errors::{AppError, AppErrorType};
use sproot::models::Alerts;

/// GET /api/alerts
/// Return all alerts
pub async fn alerts_list(
    app_data: web::Data<AppData>,
    info: web::Query<PagedInfo>,
) -> Result<HttpResponse, AppError> {
    info!("Route GET /api/alerts");
    // If size is over 500 or less than 30, return error
    let size = info.size.unwrap_or(100);
    let page = info.page.unwrap_or(0);
    if !(30..=500).contains(&size) {
        Err(AppError {
            message: Some("The size parameters must be 30 < size <= 500".to_string()),
            cause: None,
            error_type: AppErrorType::InvalidRequest,
        })
    } else {
        // use web::block to offload blocking Diesel code without blocking server thread
        let data = web::block(move || {
            Alerts::get_list_host(&app_data.metrics_db.get()?, &info.uuid, size, page)
        })
        .await??;
        // Return the data as form of JSON
        Ok(HttpResponse::Ok().json(data))
    }
}
