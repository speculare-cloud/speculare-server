use actix_web::{web, HttpResponse};
use diesel::sql_types::Text;
use diesel::{sql_query, RunQueryDsl};
use sproot::apierrors::ApiError;
use sproot::models::qtype::pct;
use sproot::models::{
    AbsDTORaw, Alerts, AlertsDTO, AlertsDTOUpdate, AlertsQuery, BaseCrud, DtoBase, ExtCrud,
    MetricsPool, PctDTORaw, QueryType, Specific,
};

use crate::api::{SpecificAlert, SpecificPaged};

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
    _metrics: web::Data<MetricsPool>,
    _info: web::Query<Specific>,
    _item: web::Json<AlertsDTO>,
) -> Result<HttpResponse, ApiError> {
    info!("Route POST /api/alerts");

    // TODO - We have to check that the query is valid

    todo!()
}

/// PATCH /api/alerts
/// Update a specific alert
pub async fn alerts_update(
    metrics: web::Data<MetricsPool>,
    info: web::Query<SpecificAlert>,
    item: web::Json<AlertsDTOUpdate>,
) -> Result<HttpResponse, ApiError> {
    info!("Route PATCH /api/alerts");

    // TODO - If something changed beside [active, name]
    // we have to check that the query is valid

    let data = web::block(move || {
        Alerts::update_and_get(&mut metrics.pool.get()?, info.id, &item.into_inner())
    })
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
    metrics: web::Data<MetricsPool>,
    item: web::Json<AlertsDTO>,
) -> Result<HttpResponse, ApiError> {
    info!("Route POST /api/alerts/test");

    let data = web::block(move || {
        let (query, qtype) = match item.construct_query() {
            Ok((q, t)) => (q, t),
            Err(err) => return Err(err),
        };

        let conn = &mut metrics.pool.get()?;
        match qtype {
            QueryType::Pct => {
                let results = sql_query(&query)
                    .bind::<Text, _>(&item.host_uuid)
                    .load::<PctDTORaw>(conn)?;
                Ok(pct::compute_pct(&results).to_string())
            }
            QueryType::Abs => {
                let results = sql_query(&query)
                    .bind::<Text, _>(&item.host_uuid)
                    .load::<AbsDTORaw>(conn)?;
                trace!("result abs is {:?}", &results);
                if results.is_empty() {
                    Ok("the result of the query (abs) is empty".to_string())
                } else {
                    Ok(results[0].value.to_string())
                }
            }
        }
    })
    .await??;

    Ok(HttpResponse::Ok().body(data))
}
