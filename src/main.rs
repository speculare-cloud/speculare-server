#[macro_use]
extern crate diesel;
#[macro_use]
extern crate log;

mod endpoints;
mod errors;
mod models_db;
mod models_http;
mod schema;

use actix_web::{middleware, App, HttpServer};
use diesel::prelude::PgConnection;
use diesel::r2d2::ConnectionManager;
use openssl::ssl::{SslAcceptor, SslFiletype, SslMethod};

pub type Pool = r2d2::Pool<ConnectionManager<PgConnection>>;

fn get_ssl_builder() -> openssl::ssl::SslAcceptorBuilder {
    let key = std::env::var("KEY_PRIV").expect("BINDING must be set");
    let cert = std::env::var("KEY_CERT").expect("BINDING must be set");
    let mut builder = SslAcceptor::mozilla_intermediate(SslMethod::tls()).unwrap();
    builder.set_private_key_file(key, SslFiletype::PEM).unwrap();
    builder.set_certificate_chain_file(cert).unwrap();

    builder
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    // Load env variable from .env
    dotenv::dotenv().ok();
    // Define the verbose of the logs - info for general and actix
    std::env::set_var("RUST_LOG", "info,actix_server=info,actix_web=info");
    // Init the log module
    env_logger::init();

    // Init the connection to the postgresql
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    // Create a pool of connection
    let pool: Pool = r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool");

    // Starting the HTTP server for dev and HTTPS for release
    let serv = HttpServer::new(move || {
        App::new()
            .wrap(middleware::Compress::default())
            .wrap(middleware::Logger::default())
            .data(pool.clone())
            // POST -> /speculare
            .service(endpoints::receiver::index)
            // GET -> /speculare
            .service(endpoints::all_host::index)
            // GET -> /speculare/{uuid}
            .service(endpoints::one_host::index)
    });

    if cfg!(debug_assertions) {
        serv.bind(std::env::var("BINDING").expect("Missing binding"))?
        .run()
        .await
    } else {
        serv.bind_openssl(
            std::env::var("BINDING").expect("Missing binding"),
            get_ssl_builder(),
        )?
        .run()
        .await
    }
}
