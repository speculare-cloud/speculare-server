use crate::data_func::db_helpers::get_data_vec;
use crate::errors::{AppError, AppErrorType};
use crate::Pool;

use actix_web::{web, HttpResponse};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct PagedInfo {
    pub size: Option<i64>,
    pub page: Option<i64>,
}

/// GET /api/speculare
/// Return all host basic informations
pub async fn index(
    db: web::Data<Pool>,
    info: web::Query<PagedInfo>,
) -> Result<HttpResponse, AppError> {
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
