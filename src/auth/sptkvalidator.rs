use actix_web::body::EitherBody;
use actix_web::dev::{self, ServiceRequest, ServiceResponse};
use actix_web::dev::{Service, Transform};
use actix_web::http::header::{HeaderName, HeaderValue};
use actix_web::{web, Error, HttpResponse};
use futures_util::future::LocalBoxFuture;
use sproot::models::{ApiKey, Specific};
use std::{
    future::{ready, Ready},
    rc::Rc,
};

use sproot::models::AuthPool;

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
            Err(_) => {
                debug!("SptkValidator: No Specific query found");
                let response = HttpResponse::BadRequest().finish().map_into_right_body();
                return Box::pin(async { Ok(ServiceResponse::new(request, response)) });
            }
        };

        // Get the MetricsPool from the server
        let auth = match request.app_data::<AuthPool>() {
            Some(auth) => auth,
            None => {
                error!("middleware: app_data is not configured correctly");
                let response = HttpResponse::InternalServerError()
                    .finish()
                    .map_into_right_body();
                return Box::pin(async { Ok(ServiceResponse::new(request, response)) });
            }
        };

        // Get a conn from the auth_db's pool
        let conn = match auth.pool.get() {
            Ok(conn) => conn,
            Err(e) => {
                error!("middleware: cannot get a auth_db connection: {}", e);
                let response = HttpResponse::InternalServerError()
                    .finish()
                    .map_into_right_body();
                return Box::pin(async { Ok(ServiceResponse::new(request, response)) });
            }
        };

        let svc = self.service.clone();
        Box::pin(async move {
            // Get the APIKEY entry corresponding to the SPTK (token)
            let api_key =
                actix_web::web::block(move || ApiKey::get_entry(&conn, sptk.to_str().unwrap()))
                    .await??;

            // If APIKEY.host_uuid is not None, we check that it's equals to
            // info.uuid (the ?uuid=XYZ) of the request. If it's the equals
            // we proceed the request, otherwise return 412 or Unauthorized
            // depending on the state of APIKEY.host_uuid.
            if let Some(khost_uuid) = api_key.host_uuid {
                if khost_uuid == info.uuid {
                    // Reconstruct the ServiceRequest to pass to the rest of Actix workers
                    let mut srv_req = ServiceRequest::from_parts(request, pl);
                    // Add SPTK_VALID (validation header) to the request, it assert
                    // that it has been passed by this middleware.
                    srv_req.headers_mut().insert(
                        HeaderName::from_static("SPTK_VALID"),
                        HeaderValue::from_static("true"),
                    );
                    let res = svc.call(srv_req);
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
