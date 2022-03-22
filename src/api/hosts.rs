use crate::server::AppData;
#[cfg(feature = "auth")]
use crate::utils::InnerUser;

use super::{Paged, Specific};

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

    // If we're in the auth feature, we need to get a list of
    // hosts belonging to the currently logged user. To do so
    // we'll fetch the ApiKey entries owned by the inner_user.uuid
    // (returning only the host_uuids).
    // Then we'll simply lookup all Host which have the host_uuid
    // from the call to the ApiKey entries.
    // This is a bit hacky, but for now it'll do the job just fine.
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

    // If we're not using the auth feature, just get the hosts using
    // the legacy method (just fetch them all, no difference for 'owner').
    #[cfg(not(feature = "auth"))]
    let data =
        web::block(move || Host::list_hosts(&app_data.metrics_db.get()?, size, page)).await??;

    Ok(HttpResponse::Ok().json(data))
}

/// POST /api/guard/hosts
/// Save data from a host into the db under his uuid
pub async fn host_ingest(
    app_data: web::Data<AppData>,
    info: web::Query<Specific>,
    item: web::Json<Vec<HttpPostHost>>,
) -> Result<HttpResponse, AppError> {
    trace!("Route POST /api/guard/hosts");

    web::block(move || Host::insert(&app_data.metrics_db.get()?, &item.into_inner(), &info.uuid))
        .await??;
    Ok(HttpResponse::Ok().finish())
}
