use crate::server::AppData;
#[cfg(feature = "auth")]
use crate::utils::InnerUser;

use super::Paged;

#[cfg(feature = "auth")]
use actix_web::web::ReqData;
use actix_web::{web, HttpResponse};
use sproot::errors::AppError;
#[cfg(feature = "auth")]
use sproot::models::ApiKey;
use sproot::models::{Host, HttpPostHost};

/// GET /api/hosts
/// Return all hosts
pub async fn host_all(
    app_data: web::Data<AppData>,
    info: web::Query<Paged>,
    #[cfg(feature = "auth")] inner_user: Option<ReqData<InnerUser>>,
) -> Result<HttpResponse, AppError> {
    trace!("Route GET /api/hosts");

    let (size, page) = info.get_size_page()?;

    #[cfg(feature = "auth")]
    let data = web::block(move || {
        let hosts_uuid = ApiKey::get_host_by_owned(
            &app_data.auth_db.get()?,
            &inner_user.unwrap().into_inner().uuid,
            size,
            page,
        )?;
        Host::get_from_uuid(&app_data.auth_db.get()?, hosts_uuid.as_slice())
    })
    .await??;

    #[cfg(not(feature = "auth"))]
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
