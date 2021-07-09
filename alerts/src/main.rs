#[macro_use]
extern crate diesel_migrations;
#[macro_use]
extern crate log;

use config::{Config, ConfigError};
use diesel::{prelude::PgConnection, r2d2::ConnectionManager};
use sproot::{errors::AppError, models::Alerts, Pool};
use std::time::Duration;

mod api;
mod routes;
mod server;
mod utils;

lazy_static::lazy_static! {
    static ref CONFIG: Config = {
        let mut config = Config::default();
        config.merge(config::File::with_name("Alerts")).unwrap();
        config
    };
}

// Lazy static of the Token from Config to use in validator
lazy_static::lazy_static! {
    static ref TOKEN: Result<String, ConfigError> = {
        CONFIG.get_str("API_TOKEN")
    };
}

// Embed migrations into the binary
embed_migrations!();

/// Start the monitoring tasks for each alarms
///
/// TODO:   - Use a mutex or somthg to be able to stop a particular alerts
///         - In case of new alerts created a task for that alerts should be started
fn launch_monitoring(pool: Pool) -> Result<(), AppError> {
    // Get the alerts from the database
    let alerts: Vec<Alerts> = Alerts::get_data(&pool.get()?, None, 9999, 0)?;

    // Foreach alerts
    for alert in alerts {
        // Spawn a new task which will do the check for that particular alerts
        let cpool = pool.clone();
        tokio::spawn(async move {
            let mut interval = tokio::time::interval(Duration::from_secs(alert.timing as u64));
            let (query, qtype) = utils::construct_query(&alert);

            let tmp_query = query.to_uppercase();
            for statement in utils::DISALLOWED_STATEMENT {
                assert!(!tmp_query.contains(statement));
            }

            loop {
                interval.tick().await;
                // Do the sanity check here
                trace!("{}: Run every {:?}", alert.name, interval.period());
                utils::execute(&query, &alert, &qtype, &cpool.get().unwrap());
            }
        });
    }

    Ok(())
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Init the logger and set the debug level correctly
    sproot::configure_logger();
    // Init the connection to the postgresql
    let database_url = CONFIG
        .get_str("DATABASE_URL")
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
