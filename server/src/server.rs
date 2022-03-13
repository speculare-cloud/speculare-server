use super::routes;
use super::CONFIG;

use actix_cors::Cors;
use actix_web::{middleware, App, HttpServer};
use sproot::Pool;

pub struct AppData {
    pub metrics_db: Pool,
    #[cfg(feature = "auth")]
    pub auth_db: Pool,
}

/// Construct and run the actix server instance
///
/// Start by initializating a link to the database. And finish by binding and running the actix serv
pub async fn server(pool: Pool, _auth_pool: Option<Pool>) -> std::io::Result<()> {
    // Construct the HttpServer instance.
    // Passing the pool of PgConnection and defining the logger / compress middleware.
    let serv = HttpServer::new(move || {
        App::new()
            .wrap(Cors::permissive())
            .wrap(middleware::Compress::default())
            .wrap(middleware::Logger::default())
            .app_data(actix_web::web::Data::new(AppData {
                metrics_db: pool.clone(),
                #[cfg(feature = "auth")]
                auth_db: _auth_pool.as_ref().unwrap().clone(),
            }))
            .configure(routes::routes)
    })
    .workers(CONFIG.workers);
    // Bind the server (https or no)
    if !CONFIG.https {
        if !cfg!(debug_assertions) {
            warn!("You're starting speculare-server as HTTP on a production build, are you sure about what you're doing ?")
        } else {
            info!("Server started as HTTP on {}", &CONFIG.binding);
        }
        serv.bind(&CONFIG.binding)?.run().await
    } else {
        info!("Server started as HTTPS on {}", &CONFIG.binding);
        let key_priv = field_isset!(CONFIG.key_priv.as_ref(), "key_priv");
        let key_cert = field_isset!(CONFIG.key_cert.as_ref(), "key_cert");
        serv.bind_rustls(&CONFIG.binding, sproot::get_ssl_builder(key_priv, key_cert))?
            .run()
            .await
    }
}
