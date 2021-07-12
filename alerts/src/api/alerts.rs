use super::PagedInfo;

use actix_web::{web, HttpResponse};
use sproot::errors::{AppError, AppErrorType};
use sproot::models::{Alerts, HttpAlerts};
use sproot::Pool;

/// GET /api/alerts
/// Return all alerts
pub async fn alerts_list(
    db: web::Data<Pool>,
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
        let data = web::block(move || Alerts::get_list(&db.get()?, info.uuid.as_ref(), size, page))
            .await??;
        // Return the data as form of JSON
        Ok(HttpResponse::Ok().json(data))
    }
}

/// POST /api/guard/alerts
/// Save data of an alerts into the db
pub async fn alerts_add(
    db: web::Data<Pool>,
    item: web::Json<Vec<HttpAlerts>>,
) -> Result<HttpResponse, AppError> {
    info!("Route POST /api/guard/alerts : {:?}", item);
    // make all insert taking advantage of web::block to offload the request thread
    web::block(move || Alerts::create_new(&db.get()?, &item.into_inner())).await??;
    // Return a 200 status code as everything went well
    Ok(HttpResponse::Ok().finish())
}

/// PATCH /api/guard/alerts/{id}
/// Save data of an alerts into the db
pub async fn alerts_update(
    db: web::Data<Pool>,
    path: web::Path<u16>,
    item: web::Json<HttpAlerts>,
) -> Result<HttpResponse, AppError> {
    let id = path.into_inner();
    info!("Route PATCH /api/guard/alerts/{} : {:?}", id, item);
    // make all insert taking advantage of web::block to offload the request thread
    web::block(move || Alerts::modify(&db.get()?, &item, id.into())).await??;
    // Return a 200 status code as everything went well
    Ok(HttpResponse::Ok().finish())
}

/// DELETE /api/guard/alerts/{id}
/// Delete an alert previously created from the database
pub async fn alerts_delete(
    db: web::Data<Pool>,
    path: web::Path<u16>,
) -> Result<HttpResponse, AppError> {
    let id = path.into_inner();
    info!("Route DELETE /api/guard/alerts/{}", id);
    // make all insert taking advantage of web::block to offload the request thread
    web::block(move || Alerts::delete(&db.get()?, id.into())).await??;
    // Return a 200 status code as everything went well
    Ok(HttpResponse::Ok().finish())
}
