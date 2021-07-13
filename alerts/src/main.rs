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

lazy_static::lazy_static! {
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
        // TODO - Change the URL of the WS
        // TODO - Either create two tokio task, one for update and one for insert
        //        or allow to have multiple query type in the PGCDC server
        let (mut ws_stream, _) =
            match connect_async("wss://cdc.speculare.cloud/ws?query=update:hosts").await {
                Ok(val) => val,
                Err(err) => {
                    error!("WS: error while connecting: \"{}\"", err);
                    return;
                }
            };
        debug!("WS: handshake completed");

        // While we have some message, read them and wait for the next one
        // this does not spam the CPU as per ws_stream.next() will block until a message is received.
        while let Some(msg) = ws_stream.next().await {
            let msg = msg.unwrap();
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
