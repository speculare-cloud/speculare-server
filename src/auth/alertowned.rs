use std::{
    future::{ready, Ready},
    rc::Rc,
};

use actix_session::SessionExt;
use actix_web::body::EitherBody;
use actix_web::dev::{self, ServiceRequest, ServiceResponse};
use actix_web::dev::{Service, Transform};
use actix_web::{web, Error, HttpResponse};
use futures_util::future::LocalBoxFuture;
use sproot::models::Alerts;
use uuid::Uuid;

use crate::{api::SpecificAlert, METRICSPOOL};

pub struct AlertOwned;

impl<S: 'static, B> Transform<S, ServiceRequest> for AlertOwned
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<EitherBody<B>>;
    type Error = Error;
    type InitError = ();
    type Transform = AlertOwnedMiddleware<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(AlertOwnedMiddleware {
            service: Rc::new(service),
        }))
    }
}
pub struct AlertOwnedMiddleware<S> {
    service: Rc<S>,
}

impl<S, B> Service<ServiceRequest> for AlertOwnedMiddleware<S>
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

        // Extract the user_id from the CookieSession
        let inner_user = match request.get_session().get::<String>("user_id") {
            Ok(Some(inner)) => inner,
            Ok(None) | Err(_) => {
                debug!("AlertOwned: No user_id in the session");
                let response = HttpResponse::BadRequest().finish().map_into_right_body();
                return Box::pin(async { Ok(ServiceResponse::new(request, response)) });
            }
        };

        // Parse the user_id into a UUID
        let uuid = match Uuid::parse_str(&inner_user) {
            Ok(uuid) => uuid,
            Err(err) => {
                debug!("AlertOwned: Invalid UUID, cannot parse ({})", err);
                let response = HttpResponse::BadRequest().finish().map_into_right_body();
                return Box::pin(async { Ok(ServiceResponse::new(request, response)) });
            }
        };

        // Construct the Specific (get the uuid) from the query_string
        let info = match web::Query::<SpecificAlert>::from_query(request.query_string()) {
            Ok(info) => info,
            Err(err) => {
                debug!("AlertOwned: No Specific query found ({})", err);
                let response = HttpResponse::BadRequest().finish().map_into_right_body();
                return Box::pin(async { Ok(ServiceResponse::new(request, response)) });
            }
        };

        // Get a conn from the metrics_db's pool
        let mut conn = match METRICSPOOL.get() {
            Ok(conn) => conn,
            Err(err) => {
                error!("middleware: cannot get a metrics_db connection: {}", err);
                let response = HttpResponse::InternalServerError()
                    .finish()
                    .map_into_right_body();
                return Box::pin(async { Ok(ServiceResponse::new(request, response)) });
            }
        };

        Box::pin(async move {
            // Check if the alert (info.id) belong to the user (uuid)
            // -> dsl_alerts.filter(cid.eq(ccid).and(id.eq(aid)))
            let exists = actix_web::web::block(move || {
                Alerts::exists_by_owner_and_id(&mut conn, &uuid, info.id)
            })
            .await??;

            match exists {
                true => {
                    let res = svc.call(ServiceRequest::from_parts(request, pl));
                    res.await.map(ServiceResponse::map_into_left_body)
                }
                false => {
                    let response = HttpResponse::Unauthorized().finish().map_into_right_body();
                    Ok(ServiceResponse::new(request, response))
                }
            }
        })
    }
}
