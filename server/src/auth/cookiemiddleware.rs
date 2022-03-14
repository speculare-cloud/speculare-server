use actix_session::UserSession;
use actix_web::body::EitherBody;
use actix_web::dev::{self, ServiceRequest, ServiceResponse};
use actix_web::dev::{Service, Transform};
use actix_web::{web, Error, HttpResponse};
use futures_util::future::LocalBoxFuture;
use sproot::models::CustomersOwning;
use std::{
    future::{ready, Ready},
    rc::Rc,
};

use crate::api::PagedInfo;
use crate::server::AppData;

pub struct CheckCookies;

impl<S: 'static, B> Transform<S, ServiceRequest> for CheckCookies
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<EitherBody<B>>;
    type Error = Error;
    type InitError = ();
    type Transform = CheckCookiesMiddleware<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(CheckCookiesMiddleware {
            service: Rc::new(service),
        }))
    }
}
pub struct CheckCookiesMiddleware<S> {
    service: Rc<S>,
}

impl<S, B> Service<ServiceRequest> for CheckCookiesMiddleware<S>
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

        // Extract the user_id from the CookieSession
        let inner_user = match request.get_session().get::<String>("user_id") {
            Ok(Some(inner)) => inner,
            Ok(None) | Err(_) => {
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

        // Construct the PagedInfo from the query_string
        let info = match web::Query::<PagedInfo>::from_query(request.query_string()) {
            Ok(info) => info,
            Err(_) => {
                let response = HttpResponse::BadRequest().finish().map_into_right_body();
                return Box::pin(async { Ok(ServiceResponse::new(request, response)) });
            }
        };

        let svc = self.service.clone();
        Box::pin(async move {
            let exists = actix_web::web::block(move || {
                CustomersOwning::entry_exists(&conn, &inner_user, &info.uuid)
            })
            .await?;

            match exists {
                Ok(true) => {
                    let res = svc.call(ServiceRequest::from_parts(request, pl));
                    res.await.map(ServiceResponse::map_into_left_body)
                }
                Ok(false) => {
                    let response = HttpResponse::Unauthorized().finish().map_into_right_body();
                    Ok(ServiceResponse::new(request, response))
                }
                Err(e) => {
                    error!("middleware: entry_exists: failed due to {}", e);
                    let response = HttpResponse::InternalServerError()
                        .finish()
                        .map_into_right_body();
                    Ok(ServiceResponse::new(request, response))
                }
            }
        })
    }
}
