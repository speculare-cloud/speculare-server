#[global_allocator]
static ALLOC: snmalloc_rs::SnMalloc = snmalloc_rs::SnMalloc;

#[macro_use]
extern crate diesel;

pub mod errors;
pub mod models;

use diesel::{prelude::PgConnection, r2d2::ConnectionManager};
use rustls::internal::pemfile::{certs, pkcs8_private_keys, rsa_private_keys};
use rustls::{NoClientAuth, ServerConfig};
use std::fs::File;
use std::io::BufReader;

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

/// Return the ServerConfig needed for Actix to be binded on HTTPS
///
/// Use key and cert for the path to find the files.
pub fn get_ssl_builder(key: String, cert: String) -> ServerConfig {
    // Init the ServerConfig with no Client's cert verifiction
    let mut config = ServerConfig::new(NoClientAuth::new());
    // Open BufReader on the key and cert files to read their content
    let cert_file = &mut BufReader::new(
        File::open(&cert).expect(&format!("Certificate file not found at {}", cert)),
    );
    let key_file =
        &mut BufReader::new(File::open(&key).expect(&format!("Key file not found at {}", key)));
    // Create a Vec of certificate by extracting all cert from cert_file
    let cert_chain = certs(cert_file).unwrap();
    // Extract all PKCS8-encoded private key from key_file and generate a Vec from them
    let mut keys = pkcs8_private_keys(key_file).unwrap();
    // If no keys are found, we try using the rsa type
    if keys.is_empty() {
        // Reopen a new BufReader as pkcs8_private_keys took over the previous one
        let key_file =
            &mut BufReader::new(File::open(&key).expect(&format!("Key file not found at {}", key)));
        keys = rsa_private_keys(key_file).unwrap();
    }
    // Set a single certificate to be used for all subsequent request
    config.set_single_cert(cert_chain, keys.remove(0)).unwrap();

    config
}
