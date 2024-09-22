use actix_web::{guard, web};
use {
    crate::auth::{
        alerthostowned::AlertHostOwned, alertowned::AlertOwned, checksessions::CheckSessions,
        sptkvalidator::SptkValidator,
    },
    sproot::get_session_middleware,
};

use crate::{
    api::{
        alerts, cpustats, cputimes, disks, hosts, incidents, ioblock, ionet, loadavg, memory, swap,
    },
    CONFIG,
};

pub fn routes(cfg: &mut web::ServiceConfig) {
    cfg.route("/ping", web::get().to(|| async { "zpour" }))
        .route("/ping", web::head().to(|| async { "zpour" }))
        .service(
            web::resource("/api/hosts")
                .guard(guard::Post())
                .wrap(SptkValidator)
                .route(web::post().to(hosts::host_ingest)),
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
                .wrap(CheckSessions)
                .wrap(get_session_middleware(
                    CONFIG.cookie_secret.as_bytes(),
                    "SP-CKS".to_string(),
                    CONFIG.cookie_domain.to_owned(),
                ))
                .route(web::get().to(hosts::host_specific)),
        )
        .service(
            web::resource("/api/incidents")
                .wrap(get_session_middleware(
                    CONFIG.cookie_secret.as_bytes(),
                    "SP-CKS".to_string(),
                    CONFIG.cookie_domain.to_owned(),
                ))
                .route(web::get().to(incidents::incidents_list)),
        )
        .service(
            web::resource("/api/alerts")
                .guard(guard::Any(guard::Patch()).or(guard::Delete()))
                .wrap(AlertOwned)
                .wrap(get_session_middleware(
                    CONFIG.cookie_secret.as_bytes(),
                    "SP-CKS".to_string(),
                    CONFIG.cookie_domain.to_owned(),
                ))
                .route(web::patch().to(alerts::alerts_update))
                .route(web::delete().to(alerts::alerts_delete)),
        )
        .service(
            web::resource("/api/alerts")
                .guard(guard::Post())
                .wrap(AlertHostOwned)
                .wrap(get_session_middleware(
                    CONFIG.cookie_secret.as_bytes(),
                    "SP-CKS".to_string(),
                    CONFIG.cookie_domain.to_owned(),
                ))
                .route(web::post().to(alerts::alerts_create)),
        )
        .service(
            web::resource("/api/alerts/test")
                .guard(guard::Post())
                .wrap(get_session_middleware(
                    CONFIG.cookie_secret.as_bytes(),
                    "SP-CKS".to_string(),
                    CONFIG.cookie_domain.to_owned(),
                ))
                .route(web::post().to(alerts::alerts_test)),
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
                .route("/ioblocks", web::get().to(ioblock::ioblocks))
                .route("/ionets", web::get().to(ionet::ionets))
                .route("/memory", web::get().to(memory::memory))
                .route("/swap", web::get().to(swap::swap))
                .route(
                    "/incidents/count",
                    web::get().to(incidents::incidents_count),
                )
                .service(
                    web::scope("/alerts")
                        .route("/count", web::get().to(alerts::alerts_count))
                        .route("", web::get().to(alerts::alerts_list)),
                ),
        );
}
