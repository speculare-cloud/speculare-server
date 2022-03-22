use crate::server::AppData;

use super::{PagedInfo, SpecificPaged};

use actix_web::{web, HttpResponse};
use sproot::errors::AppError;
use sproot::models::Disk;

/// GET /api/disks
/// Return disks for a particular host
pub async fn disks(
    app_data: web::Data<AppData>,
    info: web::Query<PagedInfo>,
) -> Result<HttpResponse, AppError> {
    trace!("Route GET /api/disks : {:?}", info);

    let data = web::block(move || {
        Disk::get_data_dated(
            &app_data.metrics_db.get()?,
            &info.uuid,
            info.min_date,
            info.max_date,
        )
    })
    .await??;

    Ok(HttpResponse::Ok().json(data))
}

/// GET /api/disks_count
/// Return disks_count for a particular host
pub async fn disks_count(
    app_data: web::Data<AppData>,
    info: web::Query<SpecificPaged>,
) -> Result<HttpResponse, AppError> {
    trace!("Route GET /api/disks_count : {:?}", info);

    let data =
        web::block(move || Disk::count(&app_data.metrics_db.get()?, &info.uuid, info.get_size()?))
            .await??;

    Ok(HttpResponse::Ok().body(data.to_string()))
}
