#[macro_use]
extern crate diesel;
#[macro_use]
extern crate log;

mod errors;
mod handlers;
mod logger;
mod models;
mod routes;
mod server;

use diesel::{prelude::PgConnection, r2d2::ConnectionManager};

pub type Pool = r2d2::Pool<ConnectionManager<PgConnection>>;

pub type ConnType = r2d2::PooledConnection<ConnectionManager<PgConnection>>;

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
