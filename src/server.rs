use actix_cors::Cors;
use actix_web::web::Data;
use actix_web::{middleware, App, HttpServer};
use sproot::apierrors::ApiError;
use sproot::models::MetricsPool;
use sproot::Pool;

use super::routes;
use super::CONFIG;

/// Construct and run the actix server instance
///
/// Start by initializing a link to the database. And finish by binding and running the actix serv
pub async fn server(pool: Pool) -> std::io::Result<()> {
    let serve = HttpServer::new(move || {
        let metrics_pool = MetricsPool { pool: pool.clone() };

        let app = App::new()
            .wrap(Cors::permissive())
            .wrap(middleware::Compress::default())
            .wrap(middleware::Logger::default())
            .app_data(Data::new(metrics_pool));

        app.configure(routes::routes)
    })
    .workers(CONFIG.workers);

    // Bind the server (https or http)
    let p = if !CONFIG.https {
        if !cfg!(debug_assertions) {
            warn!("You're starting speculare-server as HTTP on a production build, are you sure about what you're doing ?")
        }

        info!("Server started as HTTP on {}", &CONFIG.binding);
        serve.bind(&CONFIG.binding)?.run()
    } else {
        let tls_config = unwrapf!(sproot::get_ssl_builder(
            unwrapf!(field_isset!(CONFIG.key_priv.as_ref(), "key_priv")),
            unwrapf!(field_isset!(CONFIG.key_cert.as_ref(), "key_cert")),
        ));

        info!("Server started as HTTPS on {}", &CONFIG.binding);
        serve.bind_rustls_0_23(&CONFIG.binding, tls_config)?.run()
    };

    p.await
}
