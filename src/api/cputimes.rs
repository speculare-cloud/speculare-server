use crate::server::AppData;

use super::PagedInfo;

use actix_web::{web, HttpResponse};
use sproot::errors::AppError;
use sproot::models::CpuTimes;

/// GET /api/cputimes
/// Return cputimes for a particular host
pub async fn cputimes(
    app_data: web::Data<AppData>,
    info: web::Query<PagedInfo>,
) -> Result<HttpResponse, AppError> {
    trace!("Route GET /api/cputimes : {:?}", info);

    if info.is_dated() {
        let data = web::block(move || {
            CpuTimes::get_data_dated(
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
            CpuTimes::get_data(&app_data.metrics_db.get()?, &info.uuid, size, page)
        })
        .await??;
        Ok(HttpResponse::Ok().json(data))
    }
}
