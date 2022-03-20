use actix_web::body::EitherBody;
use actix_web::dev::{self, ServiceRequest, ServiceResponse};
use actix_web::dev::{Service, Transform};
use actix_web::{Error, HttpResponse};
use futures_util::future::LocalBoxFuture;
use sproot::models::ApiKey;
use std::{
    future::{ready, Ready},
    rc::Rc,
};

use crate::server::AppData;

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

        // Get the SPTK header and the SP-UUID, error if not found (400)
        let sptk = match request.headers().get("SPTK") {
            Some(sptk) => sptk.to_owned(),
            None => {
                let response = HttpResponse::BadRequest().finish().map_into_right_body();
                return Box::pin(async { Ok(ServiceResponse::new(request, response)) });
            }
        };
        let host_uuid = match request.headers().get("SP-UUID") {
            Some(host_uuid) => host_uuid.to_owned(),
            None => {
                let response = HttpResponse::BadRequest().finish().map_into_right_body();
                return Box::pin(async { Ok(ServiceResponse::new(request, response)) });
            }
        };

        // Get the AppData from the server
        let app_data = match request.app_data::<AppData>() {
            Some(app_data) => app_data,
            None => {
                error!("middleware: app_data is not configured correctly");
                let response = HttpResponse::InternalServerError()
                    .finish()
                    .map_into_right_body();
                return Box::pin(async { Ok(ServiceResponse::new(request, response)) });
            }
        };

        // Get a conn from the auth_db's pool
        let conn = match app_data.auth_db.get() {
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
            let api_key =
                actix_web::web::block(move || ApiKey::get_entry(&conn, sptk.to_str().unwrap()))
                    .await??;

            if api_key.host_uuid == host_uuid.to_str().unwrap() {
                let response = HttpResponse::Unauthorized().finish().map_into_right_body();
                Ok(ServiceResponse::new(request, response))
            } else {
                let res = svc.call(ServiceRequest::from_parts(request, pl));
                res.await.map(ServiceResponse::map_into_left_body)
            }
        })
    }
}
