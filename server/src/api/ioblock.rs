use crate::server::AppData;

use super::PagedInfo;

use actix_web::{web, HttpResponse};
use sproot::errors::AppError;
use sproot::models::IoBlock;

/// GET /api/ioblocks
/// Return ioblock for a particular host
pub async fn ioblocks(
    app_data: web::Data<AppData>,
    info: web::Query<PagedInfo>,
) -> Result<HttpResponse, AppError> {
    trace!("Route GET /api/ioblocks : {:?}", info);

    if info.is_dated() {
        let data = web::block(move || {
            IoBlock::get_data_dated(
                &app_data.metrics_db.get()?,
                &info.uuid,
                info.min_date.unwrap(),
                info.max_date.unwrap(),
            )
        })
        .await??;
        Ok(HttpResponse::Ok().json(data))
    } else {
        let (size, page) = info.get_size_page()?;
        let data = web::block(move || {
            IoBlock::get_data(&app_data.metrics_db.get()?, &info.uuid, size, page)
        })
        .await??;
        Ok(HttpResponse::Ok().json(data))
    }
}

/// GET /api/ioblocks_count
/// Return ioblocks_count for a particular host
pub async fn ioblocks_count(
    app_data: web::Data<AppData>,
    info: web::Query<PagedInfo>,
) -> Result<HttpResponse, AppError> {
    trace!("Route GET /api/ioblocks_count : {:?}", info);

    let data = web::block(move || {
        IoBlock::count(&app_data.metrics_db.get()?, &info.uuid, info.get_size()?)
    })
    .await??;
    Ok(HttpResponse::Ok().body(data.to_string()))
}
