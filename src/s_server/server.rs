use super::routes;

use crate::Pool;

use actix_cors::Cors;
use actix_web::{middleware, App, HttpServer};

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
            .data(pool.clone())
            .configure(routes::routes)
    });
    // Bind and run the server on HTTP or HTTPS depending on the mode of compilation.
    let binding = std::env::var("BINDING").expect("Missing binding");
    // Check if we should enable https
    let https = std::env::var("HTTPS");
    // Bind the server (https or no)
    if https.is_err() || https.unwrap() == "NO" {
        if !cfg!(debug_assertions) {
            warn!("You're starting speculare-server as HTTP on a production build, are you sure about what you're doing ?")
        } else {
            info!("Server started as HTTP");
        }
        serv.bind(binding)?.run().await
    } else {
        info!("Server started as HTTPS");
        serv.bind_openssl(binding, crate::get_ssl_builder())?
            .run()
            .await
    }
}
