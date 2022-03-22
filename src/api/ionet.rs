use crate::server::AppData;

use super::{PagedInfo, SpecificPaged};

use actix_web::{web, HttpResponse};
use sproot::errors::AppError;
use sproot::models::IoNet;

/// GET /api/ionets
/// Return ionets for a particular host
pub async fn ionets(
    app_data: web::Data<AppData>,
    info: web::Query<PagedInfo>,
) -> Result<HttpResponse, AppError> {
    trace!("Route GET /api/ionets : {:?}", info);

    let data = web::block(move || {
        IoNet::get_data_dated(
            &app_data.metrics_db.get()?,
            &info.uuid,
            info.min_date,
            info.max_date,
        )
    })
    .await??;

    Ok(HttpResponse::Ok().json(data))
}

/// GET /api/ionets_count
/// Return ionets_count for a particular host
pub async fn ionets_count(
    app_data: web::Data<AppData>,
    info: web::Query<SpecificPaged>,
) -> Result<HttpResponse, AppError> {
    trace!("Route GET /api/ionets_count : {:?}", info);

    let data =
        web::block(move || IoNet::count(&app_data.metrics_db.get()?, &info.uuid, info.get_size()?))
            .await??;

    Ok(HttpResponse::Ok().body(data.to_string()))
}
