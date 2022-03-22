use crate::server::AppData;

use super::SpecificDated;

use actix_web::{web, HttpResponse};
use sproot::errors::AppError;
use sproot::models::CpuStats;

/// GET /api/cpustats
/// Return cpustats for a particular host
pub async fn cpustats(
    app_data: web::Data<AppData>,
    info: web::Query<SpecificDated>,
) -> Result<HttpResponse, AppError> {
    trace!("Route GET /api/cpustats : {:?}", info);

    let data = web::block(move || {
        CpuStats::get_data_dated(
            &app_data.metrics_db.get()?,
            &info.uuid,
            info.min_date,
            info.max_date,
        )
    })
    .await??;

    Ok(HttpResponse::Ok().json(data))
}
