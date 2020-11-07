#[macro_use]
extern crate diesel;
#[macro_use]
extern crate log;

mod data;
mod data_func;
mod end_api;
mod errors;
mod handlers;
mod routes;
mod server;

use diesel::prelude::PgConnection;
use diesel::r2d2::ConnectionManager;

pub type Pool = r2d2::Pool<ConnectionManager<PgConnection>>;
pub type ConnType = r2d2::PooledConnection<diesel::r2d2::ConnectionManager<diesel::PgConnection>>;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    // Load env variable from .env
    dotenv::dotenv().ok();
    // Define the verbose of the logs - info for general and actix
    std::env::set_var("RUST_LOG", "info,actix_server=info,actix_web=info");
    // Init the log module
    env_logger::init();

    // Continue the initialization of the actix web server
    server::server().await
}
