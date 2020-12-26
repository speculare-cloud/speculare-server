#[macro_use]
extern crate diesel;
#[macro_use]
extern crate log;

mod errors;
mod handlers;
mod models;
mod routes;
mod server;
mod types;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Load env variable from .env
    dotenv::dotenv().ok();
    // Define the verbose of the logs - info for general and actix
    std::env::set_var("RUST_LOG", "info,actix_server=info,actix_web=info");
    // Init the log module
    env_logger::init();
    // Continue the initialization of the actix web server
    // And wait indefinietly for it <3
    server::server().await
}
