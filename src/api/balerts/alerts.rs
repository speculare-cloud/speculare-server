#[cfg(feature = "auth")]
use actix_session::Session;
use actix_web::{web, HttpResponse};
use ahash::AHasher;
use diesel::sql_types::Text;
use diesel::{sql_query, RunQueryDsl};
use evalexpr::eval_boolean;
use sproot::apierrors::ApiError;
use sproot::models::qtype::pct;
use sproot::models::{
    AbsDTORaw, Alerts, AlertsDTO, AlertsQuery, BaseCrud, DtoBase, ExtCrud, MetricsPool, PctDTORaw,
    QueryType,
};
use std::hash::{Hash, Hasher};

use crate::api::{SpecificAlert, SpecificPaged};
use crate::{field_changed_is_same, field_changed_is_same_opt, ALERTSHASH_CACHE};

use super::AlertsUpdate;

/// GET /api/alerts
/// Return all alerts
pub async fn alerts_list(
    metrics: web::Data<MetricsPool>,
    info: web::Query<SpecificPaged>,
) -> Result<HttpResponse, ApiError> {
    info!("Route GET /api/alerts");

    let (size, page) = info.get_size_page()?;

    let data =
        web::block(move || Alerts::get(&mut metrics.pool.get()?, &info.uuid, size, page)).await??;

    Ok(HttpResponse::Ok().json(data))
}

/// POST /api/alerts
/// Create a new alert for the specific host
pub async fn alerts_create(
    metrics: web::Data<MetricsPool>,
    item: web::Json<AlertsDTO>,
) -> Result<HttpResponse, ApiError> {
    info!("Route POST /api/alerts");

    // Compute the Hash of the Alert
    let mut hasher = AHasher::default();
    item.hash(&mut hasher);
    let hash = hasher.finish();

    // Check if the Hash already exists in the Cache
    if ALERTSHASH_CACHE.get(&hash) != Some(()) {
        return Err(ApiError::InvalidRequestError(Some(String::from(
            "the alert has not been tested so it can't be trusted",
        ))));
    }

    let data = web::block(move || Alerts::insert(&mut metrics.pool.get()?, &[item.0])).await??;

    Ok(HttpResponse::Ok().json(data))
}

/// PATCH /api/alerts
/// Update a specific alert
pub async fn alerts_update(
    metrics: web::Data<MetricsPool>,
    info: web::Query<SpecificAlert>,
    item: web::Json<AlertsUpdate>,
) -> Result<HttpResponse, ApiError> {
    info!("Route PATCH /api/alerts");

    // If something changed beside [active, name] we have to check
    // that the items changed match the whole alerts and that it's valid
    if item.update.crit.is_some()
        || item.update.lookup.is_some()
        || item.update.table.is_some()
        || item.update.warn.is_some()
        || item.update.where_clause.is_some()
    {
        field_changed_is_same_opt!(&item.update.crit, item.whole.crit, "crit")?;
        field_changed_is_same_opt!(&item.update.lookup, item.whole.lookup, "lookup")?;
        field_changed_is_same_opt!(&item.update.table, item.whole.table, "table")?;
        field_changed_is_same_opt!(&item.update.warn, item.whole.warn, "warn")?;
        field_changed_is_same!(
            &item.update.where_clause,
            item.whole.where_clause,
            "where_clause"
        )?;

        // Compute the Hash of the Alert
        let mut hasher = AHasher::default();
        item.whole.hash(&mut hasher);
        let hash = hasher.finish();

        // Check if the Hash already exists in the Cache
        if ALERTSHASH_CACHE.get(&hash) != Some(()) {
            return Err(ApiError::InvalidRequestError(Some(String::from(
                "the alert has not been tested so it can't be trusted",
            ))));
        }
    }

    let data =
        web::block(move || Alerts::update_and_get(&mut metrics.pool.get()?, info.id, &item.update))
            .await??;

    Ok(HttpResponse::Ok().json(data))
}

/// DELETE /api/alerts
/// Delete a specific alert
pub async fn alerts_delete(
    metrics: web::Data<MetricsPool>,
    info: web::Query<SpecificAlert>,
) -> Result<HttpResponse, ApiError> {
    info!("Route DELETE /api/alerts");

    let data = web::block(move || Alerts::delete(&mut metrics.pool.get()?, info.id)).await??;

    Ok(HttpResponse::Ok().body(data.to_string()))
}

/// GET /api/alerts/count
/// Return a count of incidents within size limit (or 100 if undefined)
pub async fn alerts_count(
    metrics: web::Data<MetricsPool>,
    info: web::Query<SpecificPaged>,
) -> Result<HttpResponse, ApiError> {
    info!("Route GET /api/alerts/count");

    let (size, _) = info.get_size_page()?;

    let data =
        web::block(move || Alerts::count(&mut metrics.pool.get()?, &info.uuid, size)).await??;

    Ok(HttpResponse::Ok().json(data))
}

/// GET /api/alerts/test
/// Return the result of a Alert's query if successful
pub async fn alerts_test(
    #[cfg(feature = "auth")] session: Session,
    metrics: web::Data<MetricsPool>,
    item: web::Json<AlertsDTO>,
) -> Result<HttpResponse, ApiError> {
    info!("Route POST /api/alerts/test");

    #[cfg(feature = "auth")]
    // Restrict access to auth users
    match session.get::<String>("user_id") {
        Ok(None) | Err(_) => {
            debug!("alerts_test: No user_id in the session");
            return Err(ApiError::AuthorizationError(None));
        }
        _ => (),
    };

    // Compute the Hash of the Alert
    let mut hasher = AHasher::default();
    item.hash(&mut hasher);
    let hash = hasher.finish();

    let data = web::block(move || {
        // Check if the Hash already exists in the Cache
        if ALERTSHASH_CACHE.get(&hash) == Some(()) {
            return Ok(String::from("alert is valid and already cached"));
        }

        let (query, qtype) = match item.construct_query() {
            Ok((q, t)) => (q, t),
            Err(err) => return Err(err),
        };

        let conn = &mut metrics.pool.get()?;
        let result = match qtype {
            QueryType::Pct => {
                let results = sql_query(&query)
                    .bind::<Text, _>(&item.host_uuid)
                    .load::<PctDTORaw>(conn)?;
                Ok::<(bool, std::string::String), ApiError>((
                    true,
                    pct::compute_pct(&results).to_string(),
                ))
            }
            QueryType::Abs => {
                let results = sql_query(&query)
                    .bind::<Text, _>(&item.host_uuid)
                    .load::<AbsDTORaw>(conn)?;
                trace!("result abs is {:?}", &results);
                if results.is_empty() {
                    Ok::<(bool, std::string::String), ApiError>((
                        false,
                        "the result of the query (abs) is empty".to_string(),
                    ))
                } else {
                    Ok::<(bool, std::string::String), ApiError>((
                        true,
                        results[0].value.to_string(),
                    ))
                }
            }
        }?;

        let mut val = result.1.as_str();
        if !result.0 {
            val = "0";
        }

        match eval_boolean(&item.warn.replace("$this", val)) {
            Err(e) => Err(ApiError::InvalidRequestError(Some(e.to_string()))),
            _ => Ok(()),
        }?;
        match eval_boolean(&item.crit.replace("$this", val)) {
            Err(e) => Err(ApiError::InvalidRequestError(Some(e.to_string()))),
            _ => Ok(()),
        }?;

        Ok(result.1)
    })
    .await??;

    // Insert inside the Cache
    ALERTSHASH_CACHE.insert(hash, ()).await;

    Ok(HttpResponse::Ok().body(data))
}
