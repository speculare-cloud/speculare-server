use crate::data_func::db_helpers::get_data_vec;
use crate::errors::{AppError, AppErrorType};
use crate::Pool;

use actix_identity::Identity;
use actix_web::{get, web, HttpResponse};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct PagedInfo {
    pub size: Option<i64>,
    pub page: Option<i64>,
}

#[get("/speculare")]
pub async fn index(
    id: Identity,
    db: web::Data<Pool>,
    info: web::Query<PagedInfo>,
) -> Result<HttpResponse, AppError> {
    // If the user is not identified, restrict access
    if !id.identity().is_some() {
        return Err(AppError {
            cause: None,
            message: Some("You're not allowed to access this resource".to_string()),
            error_type: AppErrorType::InvalidRequest,
        });
    }

    if log_enabled!(log::Level::Info) {
        info!("Route GET /speculare");
    }

    // If size is over 500 or less than 30, return error
    let size = info.size.unwrap_or(100);
    let page = info.page.unwrap_or(0);
    if size > 500 || size < 30 {
        Err(AppError {
            message: Some("The size parameters must be 30 < size < 500".to_string()),
            cause: None,
            error_type: AppErrorType::InvalidRequest,
        })
    } else {
        // Return the data as form of JSON
        Ok(HttpResponse::Ok().json(get_data_vec(size, page, db.get()?)?))
    }
}
