use crate::{ALERTS_LIST, RUNNING_ALERT};

use super::{analysis::execute_analysis, query::*};

use sproot::{models::Alerts, Pool};
use std::time::Duration;

/// Create the task for a particular alert and add it to the ALERTS_LIST & RUNNING_ALERT.
pub fn start_alert_task(alert: Alerts, pool: Pool) {
    let calert = alert.clone();
    // Temp value because alert is borrowed inside the tokio task later
    let alert_id = alert.id;
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
            // Execute the query and the analysis
            execute_analysis(&query, &alert, &qtype, &pool.get().unwrap());
        }
    });
    // Add the task into our AHashMap protected by RwLock (multiple readers, one write at most)
    RUNNING_ALERT.write().unwrap().insert(alert_id, alert_task);
    // Add the alert into the ALERTS_LIST
    ALERTS_LIST.write().unwrap().push(calert);
}
