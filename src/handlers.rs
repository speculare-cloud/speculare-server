use actix_web::Responder;

/// Simple route that just return "zpour" has basic health check
pub async fn health_check() -> impl Responder {
    // It's not a typo
    "zpour"
}
