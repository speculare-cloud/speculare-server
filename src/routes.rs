use crate::api;

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
                        .guard(guard::Header("SPTK", crate::TOKEN.as_ref().unwrap()))
                        .route("/hosts", web::post().to(api::hosts::host_ingest)),
                )
                .route("/hosts", web::get().to(api::hosts::host_all))
                .route("/cpustats", web::get().to(api::cpustats::cpustats))
                .route("/cputimes", web::get().to(api::cputimes::cputimes))
                .route("/loadavg", web::get().to(api::loadavg::loadavg))
                .route("/disks", web::get().to(api::disks::disks))
                .route("/disks_count", web::get().to(api::disks::disks_count))
                .route("/iostats", web::get().to(api::iostats::iostats))
                .route("/iostats_count", web::get().to(api::iostats::iostats_count))
                .route("/iocounters", web::get().to(api::iocounters::iocounters))
                .route(
                    "/iocounters_count",
                    web::get().to(api::iocounters::iocounters_count),
                )
                .route("/memory", web::get().to(api::memory::memory))
                .route("/swap", web::get().to(api::swap::swap)),
        );
}
