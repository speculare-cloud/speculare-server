use crate::api;

use actix_web::web;

// Populate the ServiceConfig with all the route needed for the server
pub fn routes(cfg: &mut web::ServiceConfig) {
    // The /ping is used only to get a status over the server
    cfg.route("/ping", web::get().to(|| async { "zpour" }))
        .route("/ping", web::head().to(|| async { "zpour" }))
        // Bind the /api/* route
        .service(
            web::scope("/api")
                // TODO - These routes should be secured behind a token verification
                // Or at least just the ingest (POST)
                .route("/hosts", web::post().to(api::hosts::host_ingest))
                .route("/hosts", web::get().to(api::hosts::host_all))
                .route("/cpustats", web::get().to(api::cpustats::cpustats))
                .route("/loadavg", web::get().to(api::loadavg::loadavg))
                .route("/disks", web::get().to(api::disks::disks))
                .route("/disks_count", web::get().to(api::disks::disks_count))
                .route("/iostats", web::get().to(api::iostats::iostats))
                .route("/iostats_count", web::get().to(api::iostats::iostats_count))
                .route("/memory", web::get().to(api::memory::memory)),
        );
}
