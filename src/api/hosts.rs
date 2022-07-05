#[cfg(feature = "auth")]
use crate::api::get_user_session;

use super::{Paged, SpecificPaged};

#[cfg(feature = "auth")]
use actix_session::Session;
use actix_web::{web, HttpResponse};
#[cfg(feature = "auth")]
use sproot::models::ApiKey;
#[cfg(feature = "auth")]
use sproot::models::AuthPool;
use sproot::models::MetricsPool;
use sproot::models::{Host, HttpPostHost};
use sproot::{apierrors::ApiError, models::Specific};

/// GET /api/hosts
/// Return all hosts
pub async fn host_all(
    metrics: web::Data<MetricsPool>,
    #[cfg(feature = "auth")] auth: web::Data<AuthPool>,
    info: web::Query<Paged>,
    #[cfg(feature = "auth")] session: Session,
) -> Result<HttpResponse, ApiError> {
    trace!("Route GET /api/hosts");

    let (size, page) = info.get_size_page()?;

    #[cfg(feature = "auth")]
    let user_uuid = get_user_session(&session)?;

    // If we're in the auth feature, we need to get a list of
    // hosts belonging to the currently logged user. To do so
    // we'll fetch the ApiKey entries owned by the inner_user.uuid
    // (returning only the host_uuids).
    // Then we'll simply lookup all Host which have the host_uuid
    // from the call to the ApiKey entries.
    // This is a bit hacky, but for now it'll do the job just fine.
    #[cfg(feature = "auth")]
    let data = web::block(move || {
        let hosts_uuid = ApiKey::get_hosts_by_owned(&mut auth.pool.get()?, &user_uuid, size, page)?;
        Host::get_from_uuids(&mut metrics.pool.get()?, hosts_uuid.as_slice())
    })
    .await??;

    // If we're not using the auth feature, just get the hosts using
    // the legacy method (just fetch them all, no difference for 'owner').
    #[cfg(not(feature = "auth"))]
    let data = web::block(move || Host::list_hosts(&mut metrics.pool.get()?, size, page)).await??;

    Ok(HttpResponse::Ok().json(data))
}

/// GET /api/host
/// Return info for a specific host
pub async fn host_specific(
    metrics: web::Data<MetricsPool>,
    info: web::Query<SpecificPaged>,
) -> Result<HttpResponse, ApiError> {
    trace!("Route GET /api/host?uuid=xyz");

    let data =
        web::block(move || Host::get_from_uuid(&mut metrics.pool.get()?, &info.uuid)).await??;

    Ok(HttpResponse::Ok().json(data))
}

/// POST /api/hosts
/// Save data from a host into the db under his uuid
pub async fn host_ingest(
    metrics: web::Data<MetricsPool>,
    info: web::Query<Specific>,
    item: web::Json<Vec<HttpPostHost>>,
) -> Result<HttpResponse, ApiError> {
    trace!("Route POST /api/guard/hosts");

    web::block(move || Host::insert(&mut metrics.pool.get()?, &item.into_inner(), &info.uuid))
        .await??;
    Ok(HttpResponse::Ok().finish())
}
