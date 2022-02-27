#[macro_use]
extern crate diesel_migrations;
#[macro_use]
extern crate log;

use ahash::AHashMap;
use config::{Config, ConfigError};
use diesel::{prelude::PgConnection, r2d2::ConnectionManager};
use lettre::{
    transport::smtp::{authentication::Credentials, PoolConfig},
    SmtpTransport,
};
use sproot::models::{Alerts, AlertsConfig};
use std::sync::{atomic::AtomicUsize, Arc};
use std::{process::exit, sync::RwLock};

use utils::{check::check_alerts_syntax, monitoring::launch_monitoring};

mod api;
mod routes;
mod server;
mod utils;

/// Evaluate an Enum into the value it hold
#[macro_export]
macro_rules! as_variant {
    ($value:expr, $variant:path) => {
        match $value {
            $variant(x) => Some(x),
            _ => None,
        }
    };
}

lazy_static::lazy_static! {
    // Lazy static of the Config which is loaded from Alerts.toml
    static ref CONFIG: Config = {
        // Get arguments
        let args: Vec<String> = std::env::args().collect();

        // Verify if we have the correct number of arguments
        if args.len() < 2 {
            println!("speculare-alerts: too few arguments: missing a \"path/to/Config.toml\"");
            exit(1);
        }

        let config_builder = Config::builder()
            .add_source(config::File::with_name(&args[1]));

        match config_builder.build() {
            Ok(conf) => conf,
            Err(e) => {
                error!("Cannot build the config: {}", e);
                exit(1);
            }
        }
    };

    // Lazy static for SmtpTransport used to send mails
    // Build it using rustls and a pool of 16 items.
    static ref MAILER: SmtpTransport = {
        let username = CONFIG
            .get_string("SMTP_USER")
            .expect("Missing SMTP_USER in the config.");
        let password = CONFIG
            .get_string("SMTP_PASSWORD")
            .expect("Missing SMTP_PASSWORD in the config.");
        let creds = Credentials::new(username, password);

        // Open a remote connection to gmail
        SmtpTransport::starttls_relay(
            &CONFIG
                .get_string("SMTP_HOST")
                .unwrap_or_else(|_| "smtp.gmail.com".into()),
        )
        .unwrap_or_else(|e| panic!("Cannot instanciate SmtpTransport due to: {}", e))
        .credentials(creds)
        .pool_config(PoolConfig::new().max_size(16))
        .build()
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
    // Lazy static of the Token from Config to use in validator
    static ref TOKEN: Result<String, ConfigError> = CONFIG.get_string("API_TOKEN");
}

// Embed migrations into the binary
embed_migrations!();

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Get arguments
    let args: Vec<String> = std::env::args().collect();
    // Verify if we have the correct number of arguments
    if args.len() == 3 && args[2] == "check" {
        sproot::configure_logger("trace".to_owned());
        check_alerts_syntax();
        exit(0);
    }

    // Init the logger and set the debug level correctly
    sproot::configure_logger(
        CONFIG
            .get_string("RUST_LOG")
            .unwrap_or_else(|_| "error,actix_server=info,actix_web=error".into()),
    );

    // Init the connection to the postgresql
    let database_url = CONFIG
        .get_string("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    // Get the max number of connection to open
    // No fear to parse it to u32 and unwrap, if not a correct value crash is ok
    let max_db_connection = CONFIG.get::<u32>("DATABASE_MAX_CONNECTION").unwrap_or(10);
    // Create a pool of connection
    // This step might spam for error max_db_connection of times, this is normal.
    let pool = r2d2::Pool::builder()
        .max_size(max_db_connection)
        .min_idle(Some((10 * max_db_connection) / 100))
        .build(manager)
        .expect("Failed to create pool");
    // Apply the migrations to the database
    // It's safe to unwrap as if there is an error at this step, we don't continue running the app
    embedded_migrations::run(
        &pool
            .get()
            .expect("Cannot get a connection from the pool for the migrations."),
    )
    .unwrap();
    // Launch the monitoring of each alarms
    launch_monitoring(pool.clone())
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e.message()))?;
    // Continue the initialization of the actix web server
    // And wait indefinietly for it <3
    server::server(pool).await
}
