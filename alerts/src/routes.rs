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
                        .route("/alerts", web::post().to(|| async { "post alerts" }))
                        .route("/alerts", web::patch().to(|| async { "patch alerts" }))
                        .route("/alerts", web::delete().to(|| async { "delete alerts" })),
                )
                .route("/alerts", web::get().to(|| async { "get alerts" })),
        );
}
