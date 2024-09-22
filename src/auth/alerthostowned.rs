use std::{
    future::{ready, Ready},
    rc::Rc,
};

use actix_session::SessionExt;
use actix_web::dev::{self, ServiceRequest, ServiceResponse};
use actix_web::{body::EitherBody, web::Json};
use actix_web::{
    dev::{Service, Transform},
    web::Bytes,
};
use actix_web::{Error, HttpResponse};
use futures_util::future::LocalBoxFuture;
use sproot::models::{AlertsDTO, ApiKey};
use uuid::Uuid;

use crate::AUTHPOOL;

use super::CHECKSESSIONS_CACHE;

pub struct AlertHostOwned;

impl<S: 'static, B> Transform<S, ServiceRequest> for AlertHostOwned
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<EitherBody<B>>;
    type Error = Error;
    type InitError = ();
    type Transform = AlertHostOwnedMiddleware<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(AlertHostOwnedMiddleware {
            service: Rc::new(service),
        }))
    }
}
pub struct AlertHostOwnedMiddleware<S> {
    service: Rc<S>,
}

impl<S, B> Service<ServiceRequest> for AlertHostOwnedMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<EitherBody<B>>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    dev::forward_ready!(service);

    fn call(&self, sres: ServiceRequest) -> Self::Future {
        let mut sres = sres;
        let svc = self.service.clone();

        // Extract the user_id from the CookieSession
        let inner_user = match sres.get_session().get::<String>("user_id") {
            Ok(Some(inner)) => inner,
            Ok(None) | Err(_) => {
                debug!("AlertHostOwned: No user_id in the session");
                let response = HttpResponse::BadRequest().finish().map_into_right_body();
                let request = sres.request().clone();
                return Box::pin(async { Ok(ServiceResponse::new(request, response)) });
            }
        };

        // Parse the user_id into a UUID
        let uuid = match Uuid::parse_str(&inner_user) {
            Ok(uuid) => uuid,
            Err(err) => {
                debug!("AlertHostOwned: Invalid UUID, cannot parse ({})", err);
                let response = HttpResponse::BadRequest().finish().map_into_right_body();
                let request = sres.request().clone();
                return Box::pin(async { Ok(ServiceResponse::new(request, response)) });
            }
        };

        // Get a conn from the metrics_db's pool
        let mut conn = match AUTHPOOL.get() {
            Ok(conn) => conn,
            Err(err) => {
                error!("middleware: cannot get a auth_db connection: {}", err);
                let response = HttpResponse::InternalServerError()
                    .finish()
                    .map_into_right_body();
                let request = sres.request().clone();
                return Box::pin(async { Ok(ServiceResponse::new(request, response)) });
            }
        };

        Box::pin(async move {
            // Extract the host_uuid and the cid of the alert
            let alert = match sres.extract::<Json<AlertsDTO>>().await {
                Ok(alert) => alert,
                Err(err) => {
                    debug!("Cannot obtain the AlertsDTO from the request: {:?}", err);
                    let response = HttpResponse::BadRequest().finish().map_into_right_body();
                    return Ok(ServiceResponse::new(sres.request().clone(), response));
                }
            };

            debug!("Got the alert: {:?}", alert);

            if alert.cid != uuid {
                let response = HttpResponse::BadRequest().finish().map_into_right_body();
                return Ok(ServiceResponse::new(sres.request().clone(), response));
            }

            if CHECKSESSIONS_CACHE.get(&alert.host_uuid) == Some(uuid) {
                trace!("CheckSessions: cache hit for {}", &alert.host_uuid);

                let encoded_alert = serde_json::to_vec(&alert.0).unwrap();
                sres.set_payload(super::bytes_to_payload(Bytes::from(encoded_alert)));
                let res = svc.call(sres);
                return res.await.map(ServiceResponse::map_into_left_body);
            }

            let host_uuid = alert.host_uuid.to_owned();
            // Check if the host (info.uuid) belong to the user (uuid)
            // -> dsl_apikeys.filter(customer_id.eq(uuid).and(host_uuid.eq(info.uuid)))
            let exists = actix_web::web::block(move || {
                ApiKey::exists_by_owner_and_host(&mut conn, &uuid, &host_uuid)
            })
            .await??;

            match exists {
                true => {
                    CHECKSESSIONS_CACHE
                        .insert(alert.host_uuid.to_owned(), uuid);

                    let encoded_alert = serde_json::to_vec(&alert.0).unwrap();
                    sres.set_payload(super::bytes_to_payload(Bytes::from(encoded_alert)));
                    let res = svc.call(sres);
                    res.await.map(ServiceResponse::map_into_left_body)
                }
                false => {
                    let response = HttpResponse::Unauthorized().finish().map_into_right_body();
                    Ok(ServiceResponse::new(sres.request().clone(), response))
                }
            }
        })
    }
}
