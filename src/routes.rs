use crate::{api, CONFIG};

use actix_web::web;

// Populate the ServiceConfig with all the route needed for the server
#[cfg(not(feature = "auth"))]
pub fn routes(cfg: &mut web::ServiceConfig) {
    use actix_web::guard;

    info!("Server configured with SPTK security");
    // The /ping is used only to get a status over the server
    cfg.route("/ping", web::get().to(|| async { "zpour" }))
        .route("/ping", web::head().to(|| async { "zpour" }))
        // Bind the /api/* route
        .service(
            web::scope("/api")
                // Guarded route by API token
                .service(
                    web::scope("/guard")
                        .guard(guard::Header("SPTK", &CONFIG.api_token))
                        .route("/hosts", web::post().to(api::hosts::host_ingest)),
                )
                .route("/hosts", web::get().to(api::hosts::host_all))
                .route("/cpustats", web::get().to(api::cpustats::cpustats))
                .route("/cputimes", web::get().to(api::cputimes::cputimes))
                .route("/loadavg", web::get().to(api::loadavg::loadavg))
                .route("/disks", web::get().to(api::disks::disks))
                .route("/disks_count", web::get().to(api::disks::disks_count))
                .route("/ioblocks", web::get().to(api::ioblock::ioblocks))
                .route(
                    "/ioblocks_count",
                    web::get().to(api::ioblock::ioblocks_count),
                )
                .route("/ionets", web::get().to(api::ionet::ionets))
                .route("/ionets_count", web::get().to(api::ionet::ionets_count))
                .route("/memory", web::get().to(api::memory::memory))
                .route("/swap", web::get().to(api::swap::swap))
                .route(
                    "/incidents",
                    web::get().to(api::balerts::incidents::incidents_list),
                )
                .route("/alerts", web::get().to(api::balerts::alerts::alerts_list)),
        );
}

#[cfg(feature = "auth")]
pub fn routes(cfg: &mut web::ServiceConfig) {
    use crate::auth::{checkcookies::CheckCookies, sptkvalidator::SptkValidator};
    use actix_session::CookieSession;

    info!("Server configured with Bearer security");
    // The /ping is used only to get a status over the server
    cfg.route("/ping", web::get().to(|| async { "zpour" }))
        .route("/ping", web::head().to(|| async { "zpour" }))
        // Bind the /api/* route
        .service(
            web::scope("/api")
                // Guarded route by API token
                .service(
                    web::scope("/guard")
                        .wrap(SptkValidator)
                        .route("/hosts", web::post().to(api::hosts::host_ingest)),
                )
                // Middleware that will validate the CookieSession
                // using the Auth server. Will extract the customer ID from the
                // Cookie and use it as a lookup to see if the asked host_uuid
                // belong to the customer.
                .wrap(CheckCookies)
                // Secure the following calls with a CookieSession
                // The cookie_secret will be shared with the Dashboard
                .wrap(CookieSession::signed(CONFIG.cookie_secret.as_bytes()).name("SP-CKS"))
                .route("/ping", web::get().to(|| async { "zpour" }))
                .route("/hosts", web::get().to(api::hosts::host_all))
                .route("/cpustats", web::get().to(api::cpustats::cpustats))
                .route("/cputimes", web::get().to(api::cputimes::cputimes))
                .route("/loadavg", web::get().to(api::loadavg::loadavg))
                .route("/disks", web::get().to(api::disks::disks))
                .route("/disks_count", web::get().to(api::disks::disks_count))
                .route("/ioblocks", web::get().to(api::ioblock::ioblocks))
                .route(
                    "/ioblocks_count",
                    web::get().to(api::ioblock::ioblocks_count),
                )
                .route("/ionets", web::get().to(api::ionet::ionets))
                .route("/ionets_count", web::get().to(api::ionet::ionets_count))
                .route("/memory", web::get().to(api::memory::memory))
                .route("/swap", web::get().to(api::swap::swap))
                .route(
                    "/incidents",
                    web::get().to(api::balerts::incidents::incidents_list),
                )
                .route("/alerts", web::get().to(api::balerts::alerts::alerts_list)),
        );
}
