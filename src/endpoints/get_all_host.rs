use crate::errors::{AppError, AppErrorType};
use crate::models_db::*;
use crate::schema::data::dsl::*;
use crate::Pool;

use actix_identity::Identity;
use actix_web::{get, web, HttpResponse};
use diesel::prelude::*;
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
    if size > 500 || size < 30 {
        Err(AppError {
            message: Some(
                "The size parameters can't be bigger than 500 and lesser than 30".to_string(),
            ),
            cause: None,
            error_type: AppErrorType::InvalidRequest,
        })
    } else {
        // Get a connection from the pool
        let conn = db.get()?;
        // Retreive the datas
        let ret: Vec<Data> = data
            .limit(size)
            .offset(info.page.unwrap_or(0) * size)
            .load(&conn)?;
        Ok(HttpResponse::Ok().json(ret))
    }
}
