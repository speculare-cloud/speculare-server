use std::{
    future::{ready, Ready},
    rc::Rc,
};

use actix_web::body::EitherBody;
use actix_web::dev::{self, ServiceRequest, ServiceResponse};
use actix_web::dev::{Service, Transform};
use actix_web::{web, Error, HttpResponse};
use futures_util::future::LocalBoxFuture;
use sproot::models::{ApiKey, Specific};

use crate::{AUTHPOOL, CONFIG};

use super::CHECKSPTK_CACHE;

pub struct SptkValidator;

impl<S: 'static, B> Transform<S, ServiceRequest> for SptkValidator
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<EitherBody<B>>;
    type Error = Error;
    type InitError = ();
    type Transform = SptkValidatorMiddleware<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(SptkValidatorMiddleware {
            service: Rc::new(service),
        }))
    }
}
pub struct SptkValidatorMiddleware<S> {
    service: Rc<S>,
}

impl<S, B> Service<ServiceRequest> for SptkValidatorMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<EitherBody<B>>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    dev::forward_ready!(service);

    fn call(&self, request: ServiceRequest) -> Self::Future {
        let (request, pl) = request.into_parts();
        let svc = self.service.clone();

        // Get the SPTK header, error if not found (400)
        let sptk = match request.headers().get("SPTK") {
            Some(sptk) => sptk.to_owned(),
            None => {
                debug!("SptkValidator: No SPTK header found");
                let response = HttpResponse::BadRequest().finish().map_into_right_body();
                return Box::pin(async { Ok(ServiceResponse::new(request, response)) });
            }
        };

        // Construct the Specific (get the uuid) from the query_string
        let info = match web::Query::<Specific>::from_query(request.query_string()) {
            Ok(info) => info,
            Err(err) => {
                debug!("SptkValidator: No Specific query found ({})", err);
                let response = HttpResponse::BadRequest().finish().map_into_right_body();
                return Box::pin(async { Ok(ServiceResponse::new(request, response)) });
            }
        };

        // Check the convertion to str and prepare to be used below
        let sptk_owned = match sptk.to_str() {
            Ok(val) => val.to_owned(),
            Err(err) => {
                debug!(
                    "SptkValidator: Couldn't change the HeaderValue to str ({})",
                    err
                );
                let response = HttpResponse::BadRequest().finish().map_into_right_body();
                return Box::pin(async { Ok(ServiceResponse::new(request, response)) });
            }
        };

        // Check if the entry exists in the cache for HOST_UUID <> USER_UUID
        if CHECKSPTK_CACHE.get(&info.uuid) == Some(sptk_owned.clone()) {
            trace!("SptkValidator: cache hit for {}", &info.uuid);
            return Box::pin(async move {
                let res = svc.call(ServiceRequest::from_parts(request, pl));
                res.await.map(ServiceResponse::map_into_left_body)
            });
        }

        // Get a conn from the auth_db's pool
        let mut conn = match AUTHPOOL.get() {
            Ok(conn) => conn,
            Err(err) => {
                error!("middleware: cannot get a auth_db connection: {}", err);
                let response = HttpResponse::InternalServerError()
                    .finish()
                    .map_into_right_body();
                return Box::pin(async { Ok(ServiceResponse::new(request, response)) });
            }
        };

        Box::pin(async move {
            let host_uuid = info.uuid.to_owned();
            // Get the APIKEY entry corresponding to the SPTK (token)
            let api_key = actix_web::web::block(move || {
                ApiKey::get_by_key_berta(&mut conn, sptk.to_str().unwrap(), &CONFIG.berta_name)
            })
            .await??;

            // If APIKEY.host_uuid is not None, we check that it's equals to
            // info.uuid (the ?uuid=XYZ) of the request. If it's the equals
            // we proceed the request, otherwise return 412 or Unauthorized
            // depending on the state of APIKEY.host_uuid.
            if let Some(khost_uuid) = api_key.host_uuid {
                if khost_uuid == info.uuid {
                    CHECKSPTK_CACHE.insert(host_uuid, sptk_owned);
                    let res = svc.call(ServiceRequest::from_parts(request, pl));
                    res.await.map(ServiceResponse::map_into_left_body)
                } else {
                    // Wrong pair of SPTK and HOST_UUID, return not authorized
                    let response = HttpResponse::Unauthorized().finish().map_into_right_body();
                    Ok(ServiceResponse::new(request, response))
                }
            } else {
                // Return 412 to signal the Client to update the field host_uuid
                // on the APIKEY (using a call to the AUTH-SSOT server).
                let response = HttpResponse::PreconditionFailed()
                    .finish()
                    .map_into_right_body();
                Ok(ServiceResponse::new(request, response))
            }
        })
    }
}
