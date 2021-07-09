#[macro_use]
extern crate diesel_migrations;
#[macro_use]
extern crate log;

use diesel::{prelude::PgConnection, r2d2::ConnectionManager};
use sproot::{errors::AppError, models::Alerts, Pool};
use std::env::VarError;
use std::time::Duration;

mod api;
mod routes;
mod server;
mod utils;

// Lazy static of the Token from .env to use in validator
lazy_static::lazy_static! {
    static ref TOKEN: Result<String, VarError> = {
        std::env::var("API_TOKEN")
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
    // let alerts: Vec<Alerts> = Alerts::get_data(&pool.get()?, None, 9999, 0)?;
    let alerts = vec![
        Alerts {
            id: 0,
            name: "cpu_usage".into(),
            table: "cputimes".into(),
            lookup: "avg abs 10m of cuser,nice,system,irq,softirq,steal over idle,iowait".into(),
            timing: 60,
            warn: "$this > 50".into(),
            crit: "$this > 80".into(),
            info: Some("average cpu utilization over the last 10 minutes".into()),
            host_uuid: "dfaa7cf24d3e46cc80e8bedc6fb77886".into(),
            where_clause: None,
        },
        Alerts {
            id: 0,
            name: "loadavg".into(),
            table: "loadavg".into(),
            lookup: "avg abs 10m of five".into(),
            timing: 60,
            warn: "$this > 50".into(),
            crit: "$this > 80".into(),
            info: Some("average of the avg cpu utilization over the last 5 minutes".into()),
            host_uuid: "dfaa7cf24d3e46cc80e8bedc6fb77886".into(),
            where_clause: None,
        },
    ];

    // Foreach alerts
    for alert in alerts {
        // Spawn a new task which will do the check for that particular alerts
        let cpool = pool.clone();
        tokio::spawn(async move {
            let mut interval = tokio::time::interval(Duration::from_secs(alert.timing as u64));
            let (query, qtype) = utils::construct_query(&alert);
            loop {
                interval.tick().await;
                // Do the sanity check here
                info!("{}: Run every {:?}", alert.name, interval.period());
                utils::execute(&query, &alert, &qtype, &cpool.get().unwrap());
            }
        });
    }

    Ok(())
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Load env variable from .env
    dotenv::dotenv().ok();
    // Init the logger and set the debug level correctly
    sproot::configure_logger();
    // Init the connection to the postgresql
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    // Get the max number of connection to open
    // No fear to parse it to u32 and unwrap, if not a correct value crash is ok
    let max_db_connection = match std::env::var("DATABASE_MAX_CONNECTION") {
        Ok(value) => value,
        Err(_) => "10".into(),
    }
    .parse::<u32>()
    .expect("Can't get the DATABASE_MAX_CONNECTION correctly, verify that it's set correctly (should be a number).");
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
