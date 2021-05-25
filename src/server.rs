use crate::routes;

use actix_cors::Cors;
use actix_web::{middleware, App, HttpServer};
use diesel::{prelude::PgConnection, r2d2::ConnectionManager};
use openssl::ssl::{SslAcceptor, SslFiletype, SslMethod};

/// Return the SslAcceptorBuilder needed for Actix to be binded on HTTPS
///
/// Use KEY_PRIV and KEY_CERT environement variable for the path to find the files.
fn get_ssl_builder() -> openssl::ssl::SslAcceptorBuilder {
    let key = std::env::var("KEY_PRIV").expect("BINDING must be set");
    let cert = std::env::var("KEY_CERT").expect("BINDING must be set");
    let mut builder = SslAcceptor::mozilla_intermediate(SslMethod::tls()).unwrap();
    builder.set_private_key_file(key, SslFiletype::PEM).unwrap();
    builder.set_certificate_chain_file(cert).unwrap();

    builder
}

/// Construct and run the actix server instance
///
/// Start by initializating a link to the database. And finish by binding and running the actix serv
pub async fn server() -> std::io::Result<()> {
    // Init the connection to the postgresql
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    // Get the max number of connection to open
    // No fear to parse it to u32 and unwrap, if not a correct value crash is ok
    let max_db_connection = match std::env::var("DATABASE_MAX_CONNECTION") {
        Ok(value) => value,
        Err(_) => "10".into(),
    }
    .parse::<u32>()
    .unwrap();
    // Create a pool of connection
    // This step might spam for error max_db_connection of times, this is normal.
    let pool = r2d2::Pool::builder()
        .max_size(max_db_connection)
        .build(manager)
        .expect("Failed to create pool");
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
        serv.bind_openssl(binding, get_ssl_builder())?.run().await
    }
}
