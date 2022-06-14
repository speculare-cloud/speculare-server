use actix_session::SessionExt;
use actix_web::body::EitherBody;
use actix_web::dev::{self, ServiceRequest, ServiceResponse};
use actix_web::dev::{Service, Transform};
use actix_web::web::Data;
use actix_web::{web, Error, HttpMessage, HttpResponse};
use futures_util::future::LocalBoxFuture;
use sproot::models::{ApiKey, AuthPool, InnerUser, Specific};
use std::{
    future::{ready, Ready},
    rc::Rc,
};
use uuid::Uuid;

pub struct CheckSessions;

impl<S: 'static, B> Transform<S, ServiceRequest> for CheckSessions
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<EitherBody<B>>;
    type Error = Error;
    type InitError = ();
    type Transform = CheckSessionsMiddleware<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(CheckSessionsMiddleware {
            service: Rc::new(service),
        }))
    }
}
pub struct CheckSessionsMiddleware<S> {
    service: Rc<S>,
}

impl<S, B> Service<ServiceRequest> for CheckSessionsMiddleware<S>
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
                debug!("CheckSessions: No user_id in the session");
                let response = HttpResponse::BadRequest().finish().map_into_right_body();
                return Box::pin(async { Ok(ServiceResponse::new(request, response)) });
            }
        };

        // Construct the Specific (get the uuid) from the query_string
        let info = match web::Query::<Specific>::from_query(request.query_string()) {
            Ok(info) => info,
            Err(_) => {
                debug!("CheckSessions: No Specific query found");
                let response = HttpResponse::BadRequest().finish().map_into_right_body();
                return Box::pin(async { Ok(ServiceResponse::new(request, response)) });
            }
        };

        // Get the AuthPool from the server
        let auth = match request.app_data::<Data<AuthPool>>() {
            Some(auth) => auth,
            None => {
                error!("middleware: auth is not configured correctly");
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
            // We parse the inner_user str to a UUID as it's the type in the database
            let uuid = Uuid::parse_str(&inner_user).unwrap();
            // Check if the host (info.uuid) belong to the user (uuid)
            // -> dsl_apikeys.filter(customer_id.eq(uuid).and(host_uuid.eq(info.uuid)))
            let exists =
                actix_web::web::block(move || ApiKey::entry_exists(&conn, &uuid, &info.uuid))
                    .await??;

            // If an entry exists, we proceed the request and add the InnerUser.
            // InnerUser is only used when getting the hosts (GET /api/hosts),
            // it allow us to query the AUTH-SSOT database with the right Uuid.
            // If the entry does not exists, return Unauthorized.
            match exists {
                true => {
                    // Add InnerUser into the extensions
                    request.extensions_mut().insert(InnerUser { uuid });

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
