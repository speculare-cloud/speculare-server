use crate::server::AppData;

use super::PagedInfo;

use actix_web::{web, HttpResponse};
use sproot::errors::AppError;
use sproot::models::{Host, HttpPostHost};

/// GET /api/hosts
/// Return all hosts
pub async fn host_all(
    app_data: web::Data<AppData>,
    info: web::Query<PagedInfo>,
) -> Result<HttpResponse, AppError> {
    trace!("Route GET /api/hosts");

    let (size, page) = info.get_size_page()?;
    let data =
        web::block(move || Host::list_hosts(&app_data.metrics_db.get()?, size, page)).await??;
    Ok(HttpResponse::Ok().json(data))
}

/// POST /api/guard/hosts
/// Save data from a host into the db under his uuid
pub async fn host_ingest(
    app_data: web::Data<AppData>,
    item: web::Json<Vec<HttpPostHost>>,
) -> Result<HttpResponse, AppError> {
    trace!("Route POST /api/guard/hosts");

    web::block(move || Host::insert(&app_data.metrics_db.get()?, &item.into_inner())).await??;
    Ok(HttpResponse::Ok().finish())
}
