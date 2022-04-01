use super::routes;
use super::CONFIG;

use actix_cors::Cors;
use actix_web::web::Data;
use actix_web::{middleware, App, HttpServer};
#[cfg(feature = "auth")]
use sproot::models::AuthPool;
use sproot::models::MetricsPool;
use sproot::{errors::AppError, Pool};

/// Construct and run the actix server instance
///
/// Start by initializing a link to the database. And finish by binding and running the actix serv
pub async fn server(pool: Pool, _auth_pool: Option<Pool>) -> std::io::Result<()> {
    let serv = HttpServer::new(move || {
        let metrics_pool = MetricsPool { pool: pool.clone() };
        #[cfg(feature = "auth")]
        let auth_pool = AuthPool {
            pool: _auth_pool.as_ref().unwrap().clone(),
        };

        let app = App::new()
            .wrap(Cors::permissive())
            .wrap(middleware::Compress::default())
            .wrap(middleware::Logger::default());

        let app = app.app_data(Data::new(metrics_pool));
        #[cfg(feature = "auth")]
        let app = app.app_data(Data::new(auth_pool));

        app.configure(routes::routes)
    })
    .workers(CONFIG.workers);

    // Bind the server (https or no)
    let server = if !CONFIG.https {
        if !cfg!(debug_assertions) {
            warn!("You're starting speculare-server as HTTP on a production build, are you sure about what you're doing ?")
        }

        info!("Server started as HTTP on {}", &CONFIG.binding);
        serv.bind(&CONFIG.binding)?.run()
    } else {
        let tls_config = match sproot::get_ssl_builder(
            field_isset!(CONFIG.key_priv.as_ref(), "key_priv").unwrap(),
            field_isset!(CONFIG.key_cert.as_ref(), "key_cert").unwrap(),
        ) {
            Ok(config) => config,
            Err(e) => {
                error!("{}", e);
                std::process::exit(1);
            }
        };

        info!("Server started as HTTPS on {}", &CONFIG.binding);
        serv.bind_rustls(&CONFIG.binding, tls_config)?.run()
    };

    // Start and wait (indefinitely)
    server.await
}
