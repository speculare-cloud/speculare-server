use super::routes;
use super::CONFIG;

use actix_cors::Cors;
use actix_web::{middleware, App, HttpServer};
use sproot::Pool;

/// Construct and run the actix server instance
///
/// Start by initializating a link to the database. And finish by binding and running the actix serv
pub async fn server(pool: Pool) -> std::io::Result<()> {
    // Construct the HttpServer instance.
    // Passing the pool of PgConnection and defining the logger / compress middleware.
    let serv = HttpServer::new(move || {
        App::new()
            .wrap(Cors::permissive())
            .wrap(middleware::Compress::default())
            .wrap(middleware::Logger::default())
            .app_data(actix_web::web::Data::new(pool.clone()))
            .configure(routes::routes)
    });
    // Bind and run the server on HTTP or HTTPS depending on the mode of compilation.
    let binding = CONFIG.get_str("BINDING").expect("Missing binding");
    // Check if we should enable https
    let https = CONFIG.get_bool("HTTPS");
    // Bind the server (https or no)
    if https.is_err() || !https.unwrap() {
        if !cfg!(debug_assertions) {
            warn!("You're starting speculare-server as HTTP on a production build, are you sure about what you're doing ?")
        } else {
            info!("Server started as HTTP");
        }
        serv.bind(binding)?.run().await
    } else {
        info!("Server started as HTTPS");
        serv.bind_rustls(
            binding,
            sproot::get_ssl_builder(
                CONFIG.get_str("KEY_PRIV").unwrap(),
                CONFIG.get_str("KEY_CERT").unwrap(),
            ),
        )?
        .run()
        .await
    }
}
