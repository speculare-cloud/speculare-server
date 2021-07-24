use super::PagedInfo;

use actix_web::{web, HttpResponse};
use sproot::errors::{AppError, AppErrorType};
use sproot::models::{Incidents, IncidentsDTOUpdate};
use sproot::Pool;

/// GET /api/incidents
/// Return all incidents
pub async fn incidents_list(
    db: web::Data<Pool>,
    info: web::Query<PagedInfo>,
) -> Result<HttpResponse, AppError> {
    info!("Route GET /api/incidents");
    // If size is over 500 or less than 30, return error
    let size = info.size.unwrap_or(100);
    let page = info.page.unwrap_or(0);
    if !(30..=500).contains(&size) {
        Err(AppError {
            message: Some("The size parameters must be 30 < size <= 500".to_string()),
            cause: None,
            error_type: AppErrorType::InvalidRequest,
        })
    } else {
        // use web::block to offload blocking Diesel code without blocking server thread
        let data =
            web::block(move || Incidents::get_list(&db.get()?, info.uuid.as_ref(), size, page))
                .await??;
        // Return the data as form of JSON
        Ok(HttpResponse::Ok().json(data))
    }
}

/// GET /api/incidents/{id}
/// Return a specific incident
pub async fn incidents_one(
    db: web::Data<Pool>,
    path: web::Path<u16>,
) -> Result<HttpResponse, AppError> {
    let id = path.into_inner();
    info!("Route GET /api/incidents/{}", id);
    // use web::block to offload blocking Diesel code without blocking server thread
    let data = web::block(move || Incidents::get(&db.get()?, id.into())).await??;
    Ok(HttpResponse::Ok().json(data))
}

/// PATCH /api/guard/incidents/{id}
/// Save data of an incidents into the db
pub async fn incidents_update(
    db: web::Data<Pool>,
    path: web::Path<u16>,
    item: web::Json<IncidentsDTOUpdate>,
) -> Result<HttpResponse, AppError> {
    let id = path.into_inner();
    info!("Route PATCH /api/guard/incidents/{} : {:?}", id, item);
    // use web::block to offload blocking Diesel code without blocking server thread
    web::block(move || Incidents::update(&db.get()?, &item, id.into())).await??;
    // Return a 200 status code as everything went well
    Ok(HttpResponse::Ok().finish())
}

// /// DELETE /api/guard/incidents/{id}
// /// Delete an alert previously created from the database
// pub async fn incidents_delete(
//     db: web::Data<Pool>,
//     path: web::Path<u16>,
// ) -> Result<HttpResponse, AppError> {
//     let id = path.into_inner();
//     info!("Route DELETE /api/guard/incidents/{}", id);
//     // use web::block to offload blocking Diesel code without blocking server thread
//     web::block(move || Incidents::delete(&db.get()?, id.into())).await??;
//     // Return a 200 status code as everything went well
//     Ok(HttpResponse::Ok().finish())
// }
