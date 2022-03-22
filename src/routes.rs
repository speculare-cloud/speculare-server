use crate::{api, CONFIG};

use actix_web::{web, Scope};

pub fn routes(cfg: &mut web::ServiceConfig) {
    // Set up ping routes
    cfg.route("/ping", web::get().to(|| async { "zpour" }))
        .route("/ping", web::head().to(|| async { "zpour" }));

    let guarded: Scope<_>;
    let mut api: Scope<_>;

    // Add scope /api/guard with fixed SPTK token
    // Add scope /api with one route /ping for debug and analysis
    #[cfg(not(feature = "auth"))]
    {
        use actix_web::guard;

        guarded = web::scope("/api/guard")
            .guard(guard::Header("SPTK", &CONFIG.api_token))
            .route("/hosts", web::post().to(api::hosts::host_ingest));

        api = web::scope("/api").route("/ping", web::get().to(|| async { "pzpour" }));
    }

    // Add scope /api/guard with database lookup for SPTK token
    // Add scope /api with one route /ping for debug and analysis but wrapped with Cookie
    #[cfg(feature = "auth")]
    {
        use crate::auth::{checkcookies::CheckCookies, sptkvalidator::SptkValidator};
        use actix_session::{
            storage::CookieSessionStore, CookieContentSecurity, SessionMiddleware,
        };

        guarded = web::scope("/api/guard")
            .wrap(SptkValidator)
            .route("/hosts", web::post().to(api::hosts::host_ingest));

        // TODO - Move to sproot and secure the cookie with domain, etc.
        let cookie_session = SessionMiddleware::builder(
            CookieSessionStore::default(),
            actix_web::cookie::Key::from(CONFIG.cookie_secret.as_bytes()),
        )
        .cookie_name("SP-CKS".to_string())
        .cookie_content_security(CookieContentSecurity::Signed)
        .build();

        api = web::scope("/api") // Middleware that will validate the CookieSession
            // using the Auth server. Will extract the customer ID from the
            // Cookie and use it as a lookup to see if the asked host_uuid
            // belong to the customer.
            .wrap(CheckCookies)
            // Secure the following calls with a CookieSession
            // The cookie_secret will be shared with the Dashboard
            .wrap(cookie_session)
            .route("/ping", web::get().to(|| async { "pzpour" }));
    }

    // Define common routes for auth and non-auth
    api = api
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
        .route("/alerts", web::get().to(api::balerts::alerts::alerts_list));

    cfg.service(guarded).service(api);
}
