use crate::end_api;
use crate::handlers;

use actix_web::web;

// Populate the ServiceConfig with all the route needed for the server
pub fn routes(cfg: &mut web::ServiceConfig) {
    // The /health is used only to get a status over the server
    cfg.route("/health", web::get().to(handlers::health_check))
        // Bind the /api/* route
        .service(
            web::scope("/api")
                .route("/speculare", web::post().to(end_api::post_one::index))
                .route("/speculare", web::get().to(end_api::get_all::index))
                .route("/speculare/{uuid}", web::get().to(end_api::get_one::index)),
        );
}
