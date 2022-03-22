use crate::api::SpecificPaged;
use crate::server::AppData;

use actix_web::{web, HttpResponse};
use sproot::errors::AppError;
use sproot::models::Incidents;

/// GET /api/incidents
/// Return all incidents
pub async fn incidents_list(
    app_data: web::Data<AppData>,
    info: web::Query<SpecificPaged>,
) -> Result<HttpResponse, AppError> {
    info!("Route GET /api/incidents");

    let (size, page) = info.get_size_page()?;

    let data = web::block(move || {
        Incidents::get_list_host(&app_data.metrics_db.get()?, &info.uuid, size, page)
    })
    .await??;

    Ok(HttpResponse::Ok().json(data))
}
