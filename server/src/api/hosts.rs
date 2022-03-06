use crate::server::AppData;

use super::PagedInfo;

use actix_web::{web, HttpResponse};
use sproot::errors::{AppError, AppErrorType};
use sproot::models::{Host, HttpPostHost};

/// GET /api/hosts
/// Return all hosts
pub async fn host_all(
    app_data: web::Data<AppData>,
    info: web::Query<PagedInfo>,
) -> Result<HttpResponse, AppError> {
    trace!("Route GET /api/hosts");

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
        let data =
            web::block(move || Host::list_hosts(&app_data.metrics_db.get()?, size, page)).await??;
        // Return the data as form of JSON
        Ok(HttpResponse::Ok().json(data))
    }
}

/// POST /api/guard/hosts
/// Save data from a host into the db under his uuid
pub async fn host_ingest(
    app_data: web::Data<AppData>,
    item: web::Json<Vec<HttpPostHost>>,
) -> Result<HttpResponse, AppError> {
    trace!("Route POST /api/guard/hosts");

    // make all insert taking advantage of web::block to offload the request thread
    web::block(move || Host::insert(&app_data.metrics_db.get()?, &item.into_inner())).await??;
    // Return a 200 status code as everything went well
    Ok(HttpResponse::Ok().finish())
}
