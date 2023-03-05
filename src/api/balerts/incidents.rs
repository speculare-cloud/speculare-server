use actix_session::Session;
use actix_web::{web, HttpResponse};
use sproot::apierrors::ApiError;
use sproot::models::Incidents;
use sproot::models::MetricsPool;
use sproot::models::ExtCrud;
use uuid::Uuid;

use crate::api::SpecificPaged;

/// GET /api/incidents
/// Return all incidents
pub async fn incidents_list(
    #[cfg(feature = "auth")] session: Session,
    metrics: web::Data<MetricsPool>,
    info: web::Query<SpecificPaged>,
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
        web::block(move || Incidents::get_owned(&mut metrics.pool.get()?, &uuid, size, page))
            .await??;

    Ok(HttpResponse::Ok().json(data))
}

/// GET /api/incidents_count
/// Return a count of incidents within size limit (or 100 if undefined)
pub async fn incidents_count(
    metrics: web::Data<MetricsPool>,
    info: web::Query<SpecificPaged>,
) -> Result<HttpResponse, ApiError> {
    info!("Route GET /api/incidents_count");

    let (size, _) = info.get_size_page()?;

    let data =
        web::block(move || Incidents::count(&mut metrics.pool.get()?, &info.uuid, size)).await??;

    Ok(HttpResponse::Ok().body(data.to_string()))
}
