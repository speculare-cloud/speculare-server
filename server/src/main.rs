#[macro_use]
extern crate diesel_migrations;
#[macro_use]
extern crate log;
#[macro_use]
extern crate sproot;

use clap::Parser;
use clap_verbosity_flag::InfoLevel;
use sproot::prog;

use crate::utils::config::Config;

mod api;
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
    verbose: clap_verbosity_flag::Verbosity<InfoLevel>,
}

// Lazy static of the Token from Config to use in validator
lazy_static::lazy_static! {
    // Lazy static of the Config which is loaded from Alerts.toml
    static ref CONFIG: Config = match Config::new() {
        Ok(config) => config,
        Err(e) => {
            error!("Cannot build the Config: {:?}", e);
            std::process::exit(1);
        }
    };
}

// Embed migrations into the binary
embed_migrations!();

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let args = Args::parse();

    // Init logger
    env_logger::Builder::new()
        .filter_module(
            &prog().unwrap_or_else(|| "alerts".to_owned()),
            args.verbose.log_level_filter(),
        )
        .init();

    flow_run::flow_run_start().await
}
