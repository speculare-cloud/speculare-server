use sproot::errors::{AppError, AppErrorType};
use sproot::models::{Alerts, HttpPostAlert};
use sproot::Pool;

use super::PagedInfo;

use actix_web::{web, HttpResponse};

/// GET /api/alerts_all
/// Return all alerts
pub async fn alerts_all(
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
        //let data = web::block(move || Host::list_hosts(&db.get()?, size, page)).await?;
        // Return the data as form of JSON
        Ok(HttpResponse::Ok().finish())
    }
}

/// POST /api/guard/alerts
/// Save data of an alerts into the db
pub async fn alerts_ingest(
    db: web::Data<Pool>,
    item: web::Json<Vec<HttpPostAlert>>,
) -> Result<HttpResponse, AppError> {
    info!("Route POST /api/guard/alerts : {:?}", item);
    // make all insert taking advantage of web::block to offload the request thread
    //web::block(move || Host::insert(&db.get()?, &item.into_inner())).await?;
    // Return a 200 status code as everything went well
    Ok(HttpResponse::Ok().finish())
}

/// PATCH /api/guard/alerts
/// Save data of an alerts into the db
pub async fn alerts_update(
    db: web::Data<Pool>,
    path: web::Path<u32>,
    item: web::Json<Vec<HttpPostAlert>>,
) -> Result<HttpResponse, AppError> {
    let id = path.into_inner();
    info!("Route PATCH /api/guard/alerts/{} : {:?}", id, item);
    // make all insert taking advantage of web::block to offload the request thread
    //web::block(move || Host::insert(&db.get()?, &item.into_inner())).await?;
    // Return a 200 status code as everything went well
    Ok(HttpResponse::Ok().finish())
}

/// DELETE /api/guard/alerts/{id}
/// Delete an alert previously created from the database
pub async fn alerts_delete(
    db: web::Data<Pool>,
    path: web::Path<u32>,
) -> Result<HttpResponse, AppError> {
    let id = path.into_inner();
    info!("Route DELETE /api/guard/alerts/{}", id);
    // make all insert taking advantage of web::block to offload the request thread
    //web::block(move || Host::insert(&db.get()?, &item.into_inner())).await?;
    // Return a 200 status code as everything went well
    Ok(HttpResponse::Ok().finish())
}
