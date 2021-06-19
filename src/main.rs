#[global_allocator]
static ALLOC: snmalloc_rs::SnMalloc = snmalloc_rs::SnMalloc;

#[macro_use]
extern crate diesel;
#[macro_use]
extern crate log;

mod api;
mod errors;
mod models;
mod routes;
mod server;

use diesel::{prelude::PgConnection, r2d2::ConnectionManager};
use std::env::VarError;

// Helper types for less boilerplate code
pub type Pool = r2d2::Pool<ConnectionManager<PgConnection>>;
pub type ConnType = r2d2::PooledConnection<ConnectionManager<PgConnection>>;

// Lazy static of the Token from .env to use in validator
lazy_static::lazy_static! {
    static ref TOKEN: Result<String, VarError> = {
        std::env::var("API_TOKEN")
    };
}

fn configure_logger() {
    // Check if the RUST_LOG already exist in the sys
    if std::env::var_os("RUST_LOG").is_none() {
        // if it doesn't, assign a default value to RUST_LOG
        // Define RUST_LOG as trace for debug and error for prod
        std::env::set_var(
            "RUST_LOG",
            if cfg!(debug_assertions) {
                "info,actix_server=info,actix_web=info"
            } else {
                "error,actix_server=error,actix_web=error"
            },
        );
    }
    // Init the logger
    env_logger::init();
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Load env variable from .env
    dotenv::dotenv().ok();
    // Init the logger and set the debug level correctly
    configure_logger();
    // Continue the initialization of the actix web server
    // And wait indefinietly for it <3
    server::server().await
}
