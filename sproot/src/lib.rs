#[global_allocator]
static ALLOC: snmalloc_rs::SnMalloc = snmalloc_rs::SnMalloc;

#[macro_use]
extern crate log;
#[macro_use]
extern crate diesel;

pub mod errors;
pub mod models;

use diesel::{prelude::PgConnection, r2d2::ConnectionManager};
use rustls::{Certificate, PrivateKey, ServerConfig};
use std::fs::File;
use std::io::BufReader;
use std::{ffi::OsStr, path::Path};

// Helper types for less boilerplate code
pub type Pool = r2d2::Pool<ConnectionManager<PgConnection>>;
pub type ConnType = r2d2::PooledConnection<ConnectionManager<PgConnection>>;

/// Evaluate an Enum into the value it hold
#[macro_export]
macro_rules! field_isset {
    ($value:expr, $name:literal) => {
        match $value {
            Some(x) => x,
            None => {
                error!(
                    "Config: optional field {} is not defined but is needed.",
                    $name
                );
                std::process::exit(1);
            }
        }
    };
}

#[macro_export]
macro_rules! as_variant {
    ($value:expr, $variant:path) => {
        match $value {
            $variant(x) => Some(x),
            _ => None,
        }
    };
}

pub fn prog() -> Option<String> {
    std::env::args()
        .next()
        .as_ref()
        .map(Path::new)
        .and_then(Path::file_name)
        .and_then(OsStr::to_str)
        .map(String::from)
}

/// Return the ServerConfig needed for Actix to be binded on HTTPS
///
/// Use key and cert for the path to find the files.
pub fn get_ssl_builder(key: &str, cert: &str) -> ServerConfig {
    // Open BufReader on the key and cert files to read their content
    let cert_file = &mut BufReader::new(
        File::open(cert).unwrap_or_else(|_| panic!("Certificate file not found at {}", cert)),
    );
    let key_file = &mut BufReader::new(
        File::open(key).unwrap_or_else(|_| panic!("Key file not found at {}", key)),
    );
    // Create a Vec of certificate by extracting all cert from cert_file
    let cert_chain = rustls_pemfile::certs(cert_file)
        .unwrap()
        .iter()
        .map(|v| Certificate(v.clone()))
        .collect();
    // Extract all PKCS8-encoded private key from key_file and generate a Vec from them
    let mut keys = rustls_pemfile::pkcs8_private_keys(key_file).unwrap();
    // If no keys are found, we try using the rsa type
    if keys.is_empty() {
        // Reopen a new BufReader as pkcs8_private_keys took over the previous one
        let key_file = &mut BufReader::new(
            File::open(&key).unwrap_or_else(|_| panic!("Key file not found at {}", key)),
        );
        keys = rustls_pemfile::rsa_private_keys(key_file).unwrap();
    }
    // Convert the first key to be a PrivateKey
    let key: PrivateKey = PrivateKey(keys.remove(0));

    // Return the ServerConfig to be used
    ServerConfig::builder()
        .with_safe_defaults()
        .with_no_client_auth()
        .with_single_cert(cert_chain, key)
        .expect("bad certificate/key")
}
