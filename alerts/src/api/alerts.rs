use super::PagedInfo;

use crate::ALERTS_LIST;

use actix_web::{web, HttpResponse};
use sproot::{
    errors::{AppError, AppErrorType},
    models::Alerts,
};

/// GET /api/alerts
/// Return all alerts
pub async fn alerts_list(info: web::Query<PagedInfo>) -> Result<HttpResponse, AppError> {
    info!("Route GET /api/alerts");
    let xyz = ALERTS_LIST.read();
    match xyz {
        Ok(content) => {
            let response = if info.uuid.is_some() {
                HttpResponse::Ok().json(
                    content
                        .iter()
                        .filter(|a| &a.host_uuid == info.uuid.as_ref().unwrap())
                        .collect::<Vec<&Alerts>>(),
                )
            } else {
                HttpResponse::Ok().json(&(*content))
            };

            Ok(response)
        }
        Err(e) => {
            error!("Cannot get the LOCK on the ALERTS_LIST: {}", e);
            Err(AppError {
                message: Some("Couldn't get the LOCK on the ALERTS_LIST".to_string()),
                cause: None,
                error_type: AppErrorType::ServerError,
            })
        }
    }
}
