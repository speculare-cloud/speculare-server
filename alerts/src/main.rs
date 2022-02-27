#[macro_use]
extern crate diesel_migrations;
#[macro_use]
extern crate log;

use ahash::AHashMap;
use clap::{Parser, Subcommand};
use clap_verbosity_flag::InfoLevel;
use sproot::models::{Alerts, AlertsConfig};
use std::sync::{atomic::AtomicUsize, Arc};
use std::{process::exit, sync::RwLock};

use crate::utils::config::Config;

mod api;
mod flow_check;
mod flow_run;
mod routes;
mod server;
mod utils;

#[derive(Parser, Debug)]
#[clap(author, version, about)]
struct Args {
    #[clap(subcommand)]
    command: Option<Commands>,

    #[clap(short = 'c', long = "config")]
    config_path: Option<String>,

    #[clap(flatten)]
    verbose: clap_verbosity_flag::Verbosity<InfoLevel>,
}

#[derive(Subcommand, Debug)]
enum Commands {
    Check,
}

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

lazy_static::lazy_static! {
    // Lazy static of the Config which is loaded from Alerts.toml
    static ref CONFIG: Config = match Config::new() {
        Ok(config) => config,
        Err(e) => {
            error!("Cannot build the Config: {:?}", e);
            exit(1);
        }
    };

    // Be warned that it is not guarantee that the task is currently running.
    // The task could have been aborted sooner due to the sanity check of the query.
    static ref RUNNING_ALERT: RwLock<AHashMap<i32, tokio::task::JoinHandle<()>>> = RwLock::new(AHashMap::new());
    // List of the Alerts (to be returned in the API call)
    static ref ALERTS_LIST: RwLock<Vec<Alerts>> = RwLock::new(Vec::new());
    // List of the AlertsConfig (to be used in the WSS)
    static ref ALERTS_CONFIG: RwLock<Vec<AlertsConfig>> = RwLock::new(Vec::new());
    // Global counter for the current ID of the Alerts
    static ref ALERTS_CURR_ID: Arc<AtomicUsize> = Arc::new(AtomicUsize::new(1));
}

// Embed migrations into the binary
embed_migrations!();

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let args = Args::parse();

    // Init logger
    env_logger::Builder::new()
        .filter_module("alerts", args.verbose.log_level_filter())
        .init();

    // Dispatch subcommands
    if let Some(Commands::Check) = &args.command {
        flow_check::flow_check_start();
        exit(0);
    }
    // Run the normal flow (start alerts, ...)
    flow_run::flow_run_start().await
}
