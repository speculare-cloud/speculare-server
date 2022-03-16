use crate::server::AppData;

use super::PagedInfo;

use actix_web::{web, HttpResponse};
use sproot::errors::AppError;
use sproot::models::CpuStats;

/// GET /api/cpustats
/// Return cpustats for a particular host
pub async fn cpustats(
    app_data: web::Data<AppData>,
    info: web::Query<PagedInfo>,
) -> Result<HttpResponse, AppError> {
    trace!("Route GET /api/cpustats : {:?}", info);

    if info.is_dated() {
        let data = web::block(move || {
            CpuStats::get_data_dated(
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
            CpuStats::get_data(&app_data.metrics_db.get()?, &info.uuid, size, page)
        })
        .await??;
        Ok(HttpResponse::Ok().json(data))
    }
}
