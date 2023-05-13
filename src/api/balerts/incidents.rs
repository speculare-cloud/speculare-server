#[cfg(feature = "auth")]
use actix_session::Session;
use actix_web::{web, HttpResponse};
use sproot::apierrors::ApiError;
#[cfg(not(feature = "auth"))]
use sproot::models::BaseCrud;
use sproot::models::ExtCrud;
use sproot::models::Incidents;
use sproot::models::MetricsPool;
#[cfg(feature = "auth")]
use uuid::Uuid;

#[cfg(feature = "auth")]
use crate::api::Paged;
use crate::api::SpecificPaged;

/// GET /api/incidents
/// Return all incidents
pub async fn incidents_list(
    #[cfg(feature = "auth")] session: Session,
    metrics: web::Data<MetricsPool>,
    #[cfg(not(feature = "auth"))] info: web::Query<SpecificPaged>,
    #[cfg(feature = "auth")] info: web::Query<Paged>,
) -> Result<HttpResponse, ApiError> {
    info!("Route GET /api/incidents");

    let (size, page) = info.get_size_page()?;

    #[cfg(not(feature = "auth"))]
    let data = web::block(move || Incidents::get(&mut metrics.pool.get()?, &info.uuid, size, page))
        .await??;

    #[cfg(feature = "auth")]
    let inner_user = match session.get::<String>("user_id") {
        Ok(Some(inner)) => inner,
        Ok(None) | Err(_) => {
            debug!("incidents_list: No user_id in the session");
            return Err(ApiError::AuthorizationError(None));
        }
    };

    #[cfg(feature = "auth")]
    let uuid = match Uuid::parse_str(&inner_user) {
        Ok(uuid) => uuid,
        Err(err) => {
            debug!("incidents_list: Invalid UUID, cannot parse ({})", err);
            return Err(ApiError::AuthorizationError(None));
        }
    };

    #[cfg(feature = "auth")]
    let data =
        web::block(move || Incidents::get_own_joined(&mut metrics.pool.get()?, &uuid, size, page))
            .await??;

    Ok(HttpResponse::Ok().json(data))
}

/// GET /api/incidents/count
/// Return a count of incidents within size limit (or 100 if undefined) for a specific host
pub async fn incidents_count(
    metrics: web::Data<MetricsPool>,
    info: web::Query<SpecificPaged>,
) -> Result<HttpResponse, ApiError> {
    info!("Route GET /api/incidents/count");

    let (size, _) = info.get_size_page()?;

    let data =
        web::block(move || Incidents::count(&mut metrics.pool.get()?, &info.uuid, size)).await??;

    Ok(HttpResponse::Ok().body(data.to_string()))
}
