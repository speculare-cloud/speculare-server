#[macro_use]
extern crate diesel_migrations;
#[macro_use]
extern crate log;
#[macro_use]
extern crate sproot;

use crate::utils::config::Config;

use clap::Parser;
use diesel_migrations::EmbeddedMigrations;
use sproot::prog;

#[cfg(feature = "auth")]
use {moka::future::Cache, sproot::Pool, std::time::Duration, uuid::Uuid};

mod api;
#[cfg(feature = "auth")]
mod auth;
mod flow_run;
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

// Lazy static of the Token from Config to use in validator
lazy_static::lazy_static! {
    // Lazy static of the Config which is loaded from the config file
    static ref CONFIG: Config = match Config::new() {
        Ok(config) => config,
        Err(e) => {
            error!("Cannot build the Config: {}", e);
            std::process::exit(1);
        }
    };
}

#[cfg(feature = "auth")]
lazy_static::lazy_static! {
    // Both cache are used to avoid looking at the auth database too often.
    // This speed up the process time required.
    // > time_to_live is set to one hour, after that the key will be evicted and
    //   we'll need to recheck it from the auth server.
    static ref CHECKSESSIONS_CACHE: Cache<String, Uuid> = Cache::builder().time_to_live(Duration::from_secs(60 * 60)).build();
    static ref CHECKSPTK_CACHE: Cache<String, String> = Cache::builder().time_to_live(Duration::from_secs(60 * 60)).build();
    static ref AUTHPOOL: Pool = flow_run::build_pool(&CONFIG.auth_database_url, CONFIG.auth_database_max_connection);
}

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

    flow_run::flow_run_start().await
}
