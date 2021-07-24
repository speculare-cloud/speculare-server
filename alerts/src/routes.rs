use super::api;

use actix_web::{guard, web};

// Populate the ServiceConfig with all the route needed for the server
pub fn routes(cfg: &mut web::ServiceConfig) {
    // The /ping is used only to get a status over the server
    cfg.route("/ping", web::get().to(|| async { "zpour" }))
        .route("/ping", web::head().to(|| async { "zpour" }))
        // Bind the /api/* route
        .service(
            web::scope("/api")
                // Guarded route by API token
                .service(
                    web::scope("/guard")
                        .guard(guard::Header("SPTK", super::TOKEN.as_ref().unwrap()))
                        .route("/alerts", web::post().to(api::alerts::alerts_add))
                        .route("/alerts/{id}", web::patch().to(api::alerts::alerts_update))
                        .route("/alerts/{id}", web::delete().to(api::alerts::alerts_delete))
                        .route(
                            "/incidents/{id}",
                            web::patch().to(api::incidents::incidents_update),
                        ),
                )
                .route("/alerts", web::get().to(api::alerts::alerts_list))
                .route("/alerts/{id}", web::get().to(api::alerts::alerts_one))
                .route("/incidents", web::get().to(api::incidents::incidents_list))
                .route(
                    "/incidents/{id}",
                    web::get().to(api::incidents::incidents_one),
                ),
        );
}
