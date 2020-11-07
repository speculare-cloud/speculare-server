use crate::end_api;
use crate::handlers;

use actix_web::web;

pub fn routes(cfg: &mut web::ServiceConfig) {
    cfg.route("/health", web::get().to(handlers::health_check))
        .service(
            web::scope("/api")
                .route("/speculare", web::post().to(end_api::post_one::index))
                .route("/speculare", web::get().to(end_api::get_all_host::index))
                .route(
                    "/speculare/{uuid}",
                    web::get().to(end_api::get_details_one::index),
                ),
        );
}
