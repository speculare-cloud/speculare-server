#[macro_use]
extern crate diesel_migrations;
#[macro_use]
extern crate log;
#[macro_use]
extern crate sproot;

use clap::Parser;
use diesel_migrations::EmbeddedMigrations;
use once_cell::sync::Lazy;
use sproot::{prog, Pool};
use utils::database::{apply_migration, build_pool};

use crate::utils::config::Config;

mod api;
#[cfg(feature = "auth")]
mod auth;
mod routes;
mod server;
mod utils;

#[derive(Parser, Debug)]
#[clap(author, version, about)]
struct Args {
    #[clap(short = 'c', long = "config")]
    config_path: Option<String>,

    #[clap(flatten)]
    verbose: clap_verbosity_flag::Verbosity,
}

// Lazy static of the Config which is loaded from the config file
static CONFIG: Lazy<Config> = Lazy::new(|| match Config::new() {
    Ok(config) => config,
    Err(e) => {
        error!("Cannot build the Config: {}", e);
        std::process::exit(1);
    }
});

#[cfg(feature = "auth")]
pub static AUTHPOOL: Lazy<Pool> = Lazy::new(|| {
    build_pool(
        &CONFIG.auth_database_url,
        CONFIG.auth_database_max_connection,
    )
});

pub static METRICSPOOL: Lazy<Pool> =
    Lazy::new(|| build_pool(&CONFIG.database_url, CONFIG.database_max_connection));

// Embed migrations into the binary
pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!();

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let args = Args::parse();

    // Define log level
    if std::env::var("RUST_LOG").is_err() {
        std::env::set_var(
            "RUST_LOG",
            format!(
                "{}={level},actix_web={level},sproot={level}",
                &prog().map_or_else(|| "speculare_server".to_owned(), |f| f.replace('-', "_")),
                level = args.verbose.log_level_filter()
            ),
        )
    }

    // Init logger/tracing
    tracing_subscriber::fmt::init();

    // Apply the migrations to the database
    apply_migration(&METRICSPOOL);

    // Continue the initialization of the Actix web server
    server::server(METRICSPOOL.clone()).await
}
