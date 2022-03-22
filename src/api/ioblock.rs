use crate::server::AppData;

use super::{PagedInfo, SpecificPaged};

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

    let data = web::block(move || {
        IoBlock::get_data_dated(
            &app_data.metrics_db.get()?,
            &info.uuid,
            info.min_date,
            info.max_date,
        )
    })
    .await??;

    Ok(HttpResponse::Ok().json(data))
}

/// GET /api/ioblocks_count
/// Return ioblocks_count for a particular host
pub async fn ioblocks_count(
    app_data: web::Data<AppData>,
    info: web::Query<SpecificPaged>,
) -> Result<HttpResponse, AppError> {
    trace!("Route GET /api/ioblocks_count : {:?}", info);

    let data = web::block(move || {
        IoBlock::count(&app_data.metrics_db.get()?, &info.uuid, info.get_size()?)
    })
    .await??;

    Ok(HttpResponse::Ok().body(data.to_string()))
}
