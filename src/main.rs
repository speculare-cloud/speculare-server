#[macro_use]
extern crate diesel;
#[macro_use]
extern crate log;

mod api;
mod errors;
mod logger;
mod models;
mod routes;
mod server;
mod validator;

use diesel::{prelude::PgConnection, r2d2::ConnectionManager};
use std::env::VarError;

pub type Pool = r2d2::Pool<ConnectionManager<PgConnection>>;

pub type ConnType = r2d2::PooledConnection<ConnectionManager<PgConnection>>;

// Lazy static of the Token from .env to use in validator
lazy_static::lazy_static! {
    static ref TOKEN: Result<String, VarError> = {
        std::env::var("API_TOKEN")
    };
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Load env variable from .env
    dotenv::dotenv().ok();
    // Init the logger and set the debug level correctly
    logger::configure();
    // Continue the initialization of the actix web server
    // And wait indefinietly for it <3
    server::server().await
}
