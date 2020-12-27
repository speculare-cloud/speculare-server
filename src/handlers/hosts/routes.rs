use crate::errors::{AppError, AppErrorType};
use crate::models::{Host, HostList, HttpPostHost};
use crate::types::Pool;

use super::PagedInfo;

use actix_web::{web, HttpResponse};

/// GET /api/hosts
/// Return all hosts's basic informations
pub async fn host_all(
    db: web::Data<Pool>,
    info: web::Query<PagedInfo>,
) -> Result<HttpResponse, AppError> {
    info!("Route GET /speculare");
    // If size is over 500 or less than 30, return error
    let size = info.size.unwrap_or(100);
    let page = info.page.unwrap_or(0);
    if !(30..=500).contains(&size) {
        Err(AppError {
            message: Some("The size parameters must be 30 < size < 500".to_string()),
            cause: None,
            error_type: AppErrorType::InvalidRequest,
        })
    } else {
        // use web::block to offload blocking Diesel code without blocking server thread
        let data = web::block(move || HostList::list(&db.get()?, size, page)).await?;
        // Return the data as form of JSON
        Ok(HttpResponse::Ok().json(data))
    }
}

/// POST /hosts
/// Save data from a host into the db under his uuid
pub async fn host_ingest(
    db: web::Data<Pool>,
    item: web::Json<Vec<HttpPostHost>>,
) -> Result<HttpResponse, AppError> {
    info!("POST /hosts : {:?}", item);
    // make all insert taking advantage of web::block to offload the request thread
    web::block(move || Host::insert(&db.get()?, &item.into_inner())).await?;
    // Return a 200 status code as everything went well
    Ok(HttpResponse::Ok().finish())
}
