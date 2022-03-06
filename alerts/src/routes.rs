use super::api;

use actix_web::web;

// Populate the ServiceConfig with all the route needed for the server
pub fn routes(cfg: &mut web::ServiceConfig) {
    // The /ping is used only to get a status over the server
    cfg.route("/ping", web::get().to(|| async { "zpour" }))
        .route("/ping", web::head().to(|| async { "zpour" }))
        // Bind the /api/* route
        .service(
            web::scope("/api")
                .route("/alerts", web::get().to(api::alerts::alerts_list))
                .route("/incidents", web::get().to(api::incidents::incidents_list))
                .route(
                    "/incidents/{id}",
                    web::get().to(api::incidents::incidents_one),
                ),
        );
}
