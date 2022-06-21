#[cfg(feature = "auth")]
use crate::auth::{checksessions::CheckSessions, sptkvalidator::SptkValidator};
use crate::{
    api::{balerts, cpustats, cputimes, disks, hosts, ioblock, ionet, loadavg, memory, swap},
    CONFIG,
};

use actix_web::{guard, web};
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
        .guard(guard::All(guard::Post()).and(guard::Header("SPTK", &CONFIG.api_token)))
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
                .route("/host", web::get().to(hosts::host_specific))
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
                .route(
                    "/incidents_count",
                    web::get().to(balerts::incidents::incidents_count),
                )
                .route("/alerts", web::get().to(balerts::alerts::alerts_list)),
        );
}

#[cfg(feature = "auth")]
pub fn routes(cfg: &mut web::ServiceConfig) {
    let mut alert_scope =
        web::scope("/alerts").route("", web::get().to(balerts::alerts::alerts_list));

    if CONFIG.alerts_source == AlertSource::Database {
        alert_scope = alert_scope
            .route("", web::post().to(balerts::alerts::alerts_create))
            .route("", web::patch().to(balerts::alerts::alerts_update))
            .route("", web::delete().to(balerts::alerts::alerts_delete));
    }

    cfg.route("/ping", web::get().to(|| async { "zpour" }))
        .route("/ping", web::head().to(|| async { "zpour" }))
        .service(
            web::scope("/api")
                .guard(guard::Post())
                .wrap(SptkValidator)
                .route("/hosts", web::post().to(hosts::host_ingest)),
        )
        .service(
            web::resource("/api/hosts")
                .wrap(get_session_middleware(
                    CONFIG.cookie_secret.as_bytes(),
                    "SP-CKS".to_string(),
                    CONFIG.cookie_domain.to_owned(),
                ))
                .route(web::get().to(hosts::host_all)),
        )
        .service(
            web::resource("/api/host")
                .wrap(get_session_middleware(
                    CONFIG.cookie_secret.as_bytes(),
                    "SP-CKS".to_string(),
                    CONFIG.cookie_domain.to_owned(),
                ))
                .route(web::get().to(hosts::host_specific)),
        )
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
                    CONFIG.cookie_domain.to_owned(),
                ))
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
                .route(
                    "/incidents_count",
                    web::get().to(balerts::incidents::incidents_count),
                )
                .service(alert_scope),
        );
}
