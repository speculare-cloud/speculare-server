#[global_allocator]
static ALLOC: snmalloc_rs::SnMalloc = snmalloc_rs::SnMalloc;

#[macro_use]
extern crate diesel;

pub mod errors;
pub mod models;

use diesel::{prelude::PgConnection, r2d2::ConnectionManager};
use openssl::ssl::{SslAcceptor, SslFiletype, SslMethod};

// Helper types for less boilerplate code
pub type Pool = r2d2::Pool<ConnectionManager<PgConnection>>;
pub type ConnType = r2d2::PooledConnection<ConnectionManager<PgConnection>>;

/// Configure the logger level for any binary calling it
pub fn configure_logger(level: String) {
    // Check if the RUST_LOG already exist in the sys
    if std::env::var_os("RUST_LOG").is_none() {
        // if it doesn't, assign a default value to RUST_LOG
        // Define RUST_LOG as trace for debug and error for prod
        std::env::set_var("RUST_LOG", level);
    }
    // Init the logger
    env_logger::init();
}

/// Return the SslAcceptorBuilder needed for Actix to be binded on HTTPS
///
/// Use KEY_PRIV and KEY_CERT environement variable for the path to find the files.
pub fn get_ssl_builder(key: String, cert: String) -> openssl::ssl::SslAcceptorBuilder {
    // Construct the SslAcceptor builder by setting the SslMethod as tls.
    let mut builder = SslAcceptor::mozilla_intermediate(SslMethod::tls()).unwrap();
    // Add the files (key & cert) to the builder
    builder.set_private_key_file(key, SslFiletype::PEM).unwrap();
    builder.set_certificate_chain_file(cert).unwrap();

    builder
}