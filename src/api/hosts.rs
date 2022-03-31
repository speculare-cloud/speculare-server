use super::Paged;

#[cfg(feature = "auth")]
use actix_web::web::ReqData;
use actix_web::{web, HttpResponse};
#[cfg(feature = "auth")]
use sproot::models::ApiKey;
#[cfg(feature = "auth")]
use sproot::models::AuthPool;
#[cfg(feature = "auth")]
use sproot::models::InnerUser;
use sproot::models::MetricsPool;
use sproot::models::{Host, HttpPostHost};
use sproot::{errors::AppError, models::Specific};

/// GET /api/hosts
/// Return all hosts
pub async fn host_all(
    metrics: web::Data<MetricsPool>,
    #[cfg(feature = "auth")] auth: web::Data<AuthPool>,
    info: web::Query<Paged>,
    #[cfg(feature = "auth")] inner_user: ReqData<InnerUser>,
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
            &auth.pool.get()?,
            &inner_user.into_inner().uuid,
            size,
            page,
        )?;
        Host::get_from_uuid(&metrics.pool.get()?, hosts_uuid.as_slice())
    })
    .await??;

    // If we're not using the auth feature, just get the hosts using
    // the legacy method (just fetch them all, no difference for 'owner').
    #[cfg(not(feature = "auth"))]
    let data = web::block(move || Host::list_hosts(&metrics.pool.get()?, size, page)).await??;

    Ok(HttpResponse::Ok().json(data))
}

/// POST /api/hosts
/// Save data from a host into the db under his uuid
pub async fn host_ingest(
    metrics: web::Data<MetricsPool>,
    info: web::Query<Specific>,
    item: web::Json<Vec<HttpPostHost>>,
) -> Result<HttpResponse, AppError> {
    trace!("Route POST /api/guard/hosts");

    web::block(move || Host::insert(&metrics.pool.get()?, &item.into_inner(), &info.uuid))
        .await??;
    Ok(HttpResponse::Ok().finish())
}
