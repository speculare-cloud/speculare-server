#[cfg(feature = "auth")]
use crate::auth::sptkvalidator::SptkValidator;
use crate::{
    api::{balerts, cpustats, cputimes, disks, hosts, ioblock, ionet, loadavg, memory, swap},
    CONFIG,
};

use actix_web::{guard, web};
#[cfg(feature = "auth")]
use sproot::check_sessions::CheckSessions;
#[cfg(feature = "auth")]
use sproot::get_session_middleware;
use sproot::models::AlertSource;

#[cfg(not(feature = "auth"))]
pub fn routes(cfg: &mut web::ServiceConfig) {
    // Extract the guard_scope construction outside of the cfg
    // because we only enable /api/alerts [POST, PATCH, DELETE]
    // if the alerts service is in the database mode (up to the user)
    // to define.
    let mut guard_scope = web::scope("/api")
        .guard(guard::Header("SPTK", &CONFIG.api_token))
        .route("/hosts", web::post().to(hosts::host_ingest));

    if CONFIG.alerts_source == AlertSource::Database {
        guard_scope = guard_scope
            .route("/alerts", web::post().to(balerts::alerts::alerts_create))
            .route("/alerts", web::patch().to(balerts::alerts::alerts_update))
            .route("/alerts", web::delete().to(balerts::alerts::alerts_delete));
    }

    cfg.route("/ping", web::get().to(|| async { "zpour" }))
        .route("/ping", web::head().to(|| async { "zpour" }))
        .service(guard_scope)
        .service(
            web::scope("/api")
                .route("/hosts", web::get().to(hosts::host_all))
                .route("/cpustats", web::get().to(cpustats::cpustats))
                .route("/cputimes", web::get().to(cputimes::cputimes))
                .route("/loadavg", web::get().to(loadavg::loadavg))
                .route("/disks", web::get().to(disks::disks))
                .route("/disks_count", web::get().to(disks::disks_count))
                .route("/ioblocks", web::get().to(ioblock::ioblocks))
                .route("/ioblocks_count", web::get().to(ioblock::ioblocks_count))
                .route("/ionets", web::get().to(ionet::ionets))
                .route("/ionets_count", web::get().to(ionet::ionets_count))
                .route("/memory", web::get().to(memory::memory))
                .route("/swap", web::get().to(swap::swap))
                .route(
                    "/incidents",
                    web::get().to(balerts::incidents::incidents_list),
                )
                .route("/alerts", web::get().to(balerts::alerts::alerts_list)),
        );
}

#[cfg(feature = "auth")]
pub fn routes(cfg: &mut web::ServiceConfig) {
    // Extract the guard_scope construction outside of the cfg
    // because we only enable /api/alerts [POST, PATCH, DELETE]
    // if the alerts service is in the database mode (up to the user)
    // to define.
    let mut guard_scope = web::scope("/api")
        .wrap(SptkValidator)
        .guard(guard::Header("SPTK_VALID", "true"))
        .route("/hosts", web::post().to(hosts::host_ingest));

    if CONFIG.alerts_source == AlertSource::Database {
        guard_scope = guard_scope
            .route("/alerts", web::post().to(balerts::alerts::alerts_create))
            .route("/alerts", web::patch().to(balerts::alerts::alerts_update))
            .route("/alerts", web::delete().to(balerts::alerts::alerts_delete));
    }

    cfg.route("/ping", web::get().to(|| async { "zpour" }))
        .route("/ping", web::head().to(|| async { "zpour" }))
        .service(guard_scope)
        .service(
            web::scope("/api")
                // Middleware that will validate the CookieSession
                // using the Auth server. Will extract the customer ID from the
                // Cookie and use it as a lookup to see if the asked host_uuid
                // belong to the customer.
                .wrap(CheckSessions)
                // Secure the following calls with a CookieSession
                // The cookie_secret will be shared with the Dashboard
                .wrap(get_session_middleware(
                    CONFIG.cookie_secret.as_bytes(),
                    "SP-CKS".to_string(),
                ))
                .route("/hosts", web::get().to(hosts::host_all))
                .route("/cpustats", web::get().to(cpustats::cpustats))
                .route("/cputimes", web::get().to(cputimes::cputimes))
                .route("/loadavg", web::get().to(loadavg::loadavg))
                .route("/disks", web::get().to(disks::disks))
                .route("/disks_count", web::get().to(disks::disks_count))
                .route("/ioblocks", web::get().to(ioblock::ioblocks))
                .route("/ioblocks_count", web::get().to(ioblock::ioblocks_count))
                .route("/ionets", web::get().to(ionet::ionets))
                .route("/ionets_count", web::get().to(ionet::ionets_count))
                .route("/memory", web::get().to(memory::memory))
                .route("/swap", web::get().to(swap::swap))
                .route(
                    "/incidents",
                    web::get().to(balerts::incidents::incidents_list),
                )
                .route("/alerts", web::get().to(balerts::alerts::alerts_list)),
        );
}
