use super::PagedInfo;

use actix_web::{web, HttpResponse};
use sproot::errors::{AppError, AppErrorType};
use sproot::models::Incidents;
use sproot::Pool;

/// GET /api/incidents
/// Return all incidents
pub async fn incidents_list(
    db: web::Data<Pool>,
    info: web::Query<PagedInfo>,
) -> Result<HttpResponse, AppError> {
    info!("Route GET /api/incidents");
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
            web::block(move || Incidents::get_list(&db.get()?, info.uuid.as_ref(), size, page))
                .await??;
        // Return the data as form of JSON
        Ok(HttpResponse::Ok().json(data))
    }
}

/// GET /api/incidents/{id}
/// Return a specific incident
pub async fn incidents_one(
    db: web::Data<Pool>,
    path: web::Path<u16>,
) -> Result<HttpResponse, AppError> {
    let id = path.into_inner();
    info!("Route GET /api/incidents/{}", id);
    // use web::block to offload blocking Diesel code without blocking server thread
    let data = web::block(move || Incidents::get(&db.get()?, id.into())).await??;
    Ok(HttpResponse::Ok().json(data))
}
