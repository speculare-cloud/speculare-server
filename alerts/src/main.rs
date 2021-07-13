#[macro_use]
extern crate diesel_migrations;
#[macro_use]
extern crate log;

use config::{Config, ConfigError};
use diesel::{prelude::PgConnection, r2d2::ConnectionManager};
use futures_util::StreamExt;
use sproot::{errors::AppError, models::Alerts, Pool};
use std::{collections::HashMap, sync::RwLock, time::Duration};
use tokio_tungstenite::connect_async;

mod api;
mod routes;
mod server;
mod utils;

// Lazy static of the Config which is loaded from Alerts.toml
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

// Lazy static holding the Alerts that are currently running,
// with their task (allow us to abort them if needed)
lazy_static::lazy_static! {
    // Be warned that it is not guarantee that the task is currently running.
    // The task could have been aborted sooner due to the sanity check of the query.
    static ref ALERTS_LIST: RwLock<HashMap<i32, tokio::task::JoinHandle<()>>> = RwLock::new(HashMap::new());
}

// Embed migrations into the binary
embed_migrations!();

/// Create the task for a particular alert and add it to the ALERTS_LIST.
fn launch_alert_task(alert: Alerts, pool: Pool) {
    // Temp value because alert is borrowed inside the tokio task later
    let alert_id = alert.id;
    // Spawn a new task which will do the check for that particular alerts
    // Save the JoinHandle so we can abort if needed later
    let alert_task: tokio::task::JoinHandle<()> = tokio::spawn(async move {
        // Construct the interval corresponding to this alert
        let mut interval = tokio::time::interval(Duration::from_secs(alert.timing as u64));
        // Construct the query and get the type of query we have
        let (query, qtype) = utils::construct_query(&alert);
        // Assert that we don't have any malicious statement in the query
        // by changing it to uppercase and checking against our list of banned statement.
        let tmp_query = query.to_uppercase();
        for statement in utils::DISALLOWED_STATEMENT {
            if tmp_query.contains(statement) {
                error!(
                    "Alerts[{}] contains disallowed statement \"{}\"",
                    alert.id, statement
                );
                return;
            }
        }

        // Start the real "forever" loop
        loop {
            // Wait for the next tick of our interval
            interval.tick().await;
            trace!("{}: Run every {:?}", alert.name, interval.period());
            // Execute the query and the analysis
            utils::execute(&query, &alert, &qtype, &pool.get().unwrap());
        }
    });
    // Add information into our HashMap protected by RwLock (multiple readers, one write at most)
    ALERTS_LIST.write().unwrap().insert(alert_id, alert_task);
}

/// Start the monitoring tasks for each alarms
///
/// TODO:   - In case of new alerts created a task for that alerts should be started
fn launch_monitoring(pool: Pool) -> Result<(), AppError> {
    // Get the alerts from the database currently present
    let alerts: Vec<Alerts> = Alerts::get_list(&pool.get()?, None, 9999, 0)?;

    // Foreach alerts
    for alert in alerts {
        // Call the function responsible for the creation of the task
        launch_alert_task(alert, pool.clone())
    }

    // Create a WS client that will connect to the Websocket of PGCDC about Alerts
    // This client will abort a task of an Alarm that is being updated and restart it
    // and will also create new task for new Alarms that are just being created after the startup.
    tokio::spawn(async {
        let domain = CONFIG
            .get_str("WSS_DOMAIN")
            .expect("Missing WSS_DOMAIN in the config file.");
        // Connect to the WS for the update type
        let (mut ws_update, _) =
            match connect_async(format!("wss://{}/ws?query=update:alerts", domain)).await {
                Ok(val) => {
                    debug!("WS: update handshake completed");
                    val
                }
                Err(err) => {
                    error!("WS: error while connecting update: \"{}\"", err);
                    // TODO - Check, return should exit the task
                    return;
                }
            };

        // Connect to the WS for the insert type
        let (mut ws_insert, _) =
            match connect_async(format!("wss://{}/ws?query=update:alerts", domain)).await {
                Ok(val) => {
                    debug!("WS: insert handshake completed");
                    val
                }
                Err(err) => {
                    error!("WS: error while connecting insert: \"{}\"", err);
                    // TODO - Check, return should exit the task
                    return;
                }
            };

        // While we have some message, read them and wait for the next one
        // We also combine both stream into "one", this is not really true but
        // we do poll both of them using tokio::select! macro.
        while let Some(msg) = tokio::select! {
            v = ws_update.next() => v,
            v = ws_insert.next() => v,
        } {
            // TODO - Handle error
            let msg = msg.unwrap();
            // Check if the message we got is text and not binary, ping, ...
            if msg.is_text() {
                // TODO - Decode and handle update/insert
                debug!("WS: Message received: \"{}\"", msg);
            }
        }
    });

    Ok(())
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Init the logger and set the debug level correctly
    sproot::configure_logger(
        CONFIG
            .get_str("RUST_LOG")
            .unwrap_or_else(|_| "error,actix_server=info,actix_web=error".into()),
    );
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
