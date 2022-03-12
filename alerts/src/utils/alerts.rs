use crate::{CONFIG, RUNNING_ALERT};

use super::{analysis::execute_analysis, config::AlertSource, query::*};

use sproot::{models::Alerts, ConnType, Pool};
use std::{mem::MaybeUninit, time::Duration};

/// Create the task for a particular alert and add it to the RUNNING_ALERT.
pub fn start_alert_task(alert: Alerts, pool: Pool) {
    // Get a conn if we're in Files AlertSource
    let mut conn = MaybeUninit::<ConnType>::uninit();
    if CONFIG.alerts_source == AlertSource::Files {
        match pool.get() {
            Ok(pconn) => conn.write(pconn),
            Err(e) => {
                error!(
                    "Cannot get a connection to the pool when start_alert_task: {}",
                    e
                );
                std::process::exit(1);
            }
        };
    }

    // Clone the alert to be used inside the RUNNING_ALERT below
    let calert = alert.clone();

    // Spawn a new task which will do the check for that particular alerts
    // Save the JoinHandle so we can abort if needed later
    let alert_task: tokio::task::JoinHandle<()> = tokio::spawn(async move {
        // Construct the interval corresponding to this alert
        let mut interval = tokio::time::interval(Duration::from_secs(alert.timing as u64));
        // Construct the query and get the type of query we have
        let (query, qtype) = match construct_query(&alert) {
            Ok((query, qtype)) => (query, qtype),
            Err(e) => {
                error!(
                    "Alert {} for host_uuid {:.6} cannot build the query: {}",
                    alert.name, alert.host_uuid, e
                );
                std::process::exit(1);
            }
        };

        // Start the real "forever" loop
        loop {
            // Wait for the next tick of our interval
            interval.tick().await;
            trace!(
                "Alert {} for host_uuid {:.6} running every {:?}",
                alert.name,
                alert.host_uuid,
                interval.period()
            );

            let conn = match pool.get() {
                Ok(conn) => conn,
                Err(e) => {
                    error!("Cannot get a connection to the pool: {}", e);
                    continue;
                }
            };
            // Execute the query and the analysis
            execute_analysis(&query, &alert, &qtype, &conn);
        }
    });
    // Add the task into our AHashMap protected by RwLock (multiple readers, one write at most)
    RUNNING_ALERT
        .write()
        .unwrap()
        .insert(calert.id.clone(), alert_task);

    // Add the Alert to the database if we're in Files mode
    if CONFIG.alerts_source == AlertSource::Files {
        let alert_id = calert.id.clone();
        match Alerts::insert(&unsafe { conn.assume_init() }, &[calert]) {
            Ok(_) => info!("Alert {} added to the database", alert_id),
            Err(e) => {
                error!("Cannot add the alerts to the database: {}", e);
                std::process::exit(1);
            }
        };
    }
}
