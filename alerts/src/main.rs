#[macro_use]
extern crate diesel_migrations;
#[macro_use]
extern crate log;

use crate::utils::config::Config;

use ahash::AHashMap;
use clap::{Parser, Subcommand};
use clap_verbosity_flag::WarnLevel;
use diesel::{prelude::PgConnection, r2d2::ConnectionManager};
use sproot::models::AlertsConfig;
use sproot::prog;
use std::sync::RwLock;

mod flow_check;
mod flow_run;
mod utils;

#[derive(Parser, Debug)]
#[clap(author, version, about)]
struct Args {
    #[clap(subcommand)]
    command: Option<Commands>,

    #[clap(short = 'c', long = "config")]
    config_path: Option<String>,

    #[clap(flatten)]
    verbose: clap_verbosity_flag::Verbosity<WarnLevel>,
}

#[derive(Subcommand, Debug)]
enum Commands {
    Check,
}

lazy_static::lazy_static! {
    // Lazy static of the Config which is loaded from the config file
    static ref CONFIG: Config = match Config::new() {
        Ok(config) => config,
        Err(e) => {
            error!("Cannot build the Config: {}", e);
            std::process::exit(1);
        }
    };

    // Be warned that it is not guarantee that the task is currently running.
    // The task could have been aborted sooner due to the sanity check of the query.
    static ref RUNNING_ALERT: RwLock<AHashMap<String, tokio::task::JoinHandle<()>>> = RwLock::new(AHashMap::new());
    // List of the AlertsConfig (to be used in the WSS)
    static ref ALERTS_CONFIG: RwLock<Vec<AlertsConfig>> = RwLock::new(Vec::new());
}

// Embed migrations into the binary
embed_migrations!();

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let args = Args::parse();

    // Init logger
    env_logger::Builder::new()
        .filter_module(
            &prog().map_or_else(|| "speculare_alerts".to_owned(), |f| f.replace('-', "_")),
            args.verbose.log_level_filter(),
        )
        .init();

    // Init the connection to the postgresql
    let manager = ConnectionManager::<PgConnection>::new(&CONFIG.database_url);
    // This step might spam for error CONFIG.database_max_connection of times, this is normal.
    let pool = match r2d2::Pool::builder()
        .max_size(CONFIG.database_max_connection)
        .min_idle(Some((10 * CONFIG.database_max_connection) / 100))
        .build(manager)
    {
        Ok(pool) => {
            info!("R2D2 PostgreSQL pool created");
            pool
        }
        Err(e) => {
            error!("Failed to create db pool: {}", e);
            std::process::exit(1);
        }
    };

    debug!("AlertSource is defined to {:?}", CONFIG.alerts_source);

    // Dispatch subcommands
    if let Some(Commands::Check) = &args.command {
        flow_check::flow_check_start(pool);
        std::process::exit(0);
    }

    // Run the normal flow (start alerts, ...)
    flow_run::flow_run_start(pool).await
}
