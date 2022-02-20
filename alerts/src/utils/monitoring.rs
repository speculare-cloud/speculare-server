use super::{analysis::execute_analysis, query::*};
use crate::{ALERTS_LIST, CONFIG, RUNNING_ALERT};

use sproot::{errors::AppError, models::Alerts, Pool};
use std::time::Duration;
use walkdir::WalkDir;

/// Create the task for a particular alert and add it to the ALERTS_LIST & RUNNING_ALERT.
fn launch_alert_task(alert: Alerts, pool: Pool) {
    let calert = alert.clone();
    // Temp value because alert is borrowed inside the tokio task later
    let alert_id = alert.id;
    // Spawn a new task which will do the check for that particular alerts
    // Save the JoinHandle so we can abort if needed later
    let alert_task: tokio::task::JoinHandle<()> = tokio::spawn(async move {
        // Construct the interval corresponding to this alert
        let mut interval = tokio::time::interval(Duration::from_secs(alert.timing as u64));
        // Construct the query and get the type of query we have
        let (query, qtype) = construct_query(&alert);
        // Assert that we don't have any malicious statement in the query
        // by changing it to uppercase and checking against our list of banned statement.
        let tmp_query = query.to_uppercase();
        for statement in super::DISALLOWED_STATEMENT {
            if tmp_query.contains(statement) {
                error!(
                    "Alerts[{}] contains disallowed statement \"{}\"",
                    alert_id, statement
                );
                return;
            }
        }

        // Start the real "forever" loop
        loop {
            // Wait for the next tick of our interval
            interval.tick().await;
            trace!(
                "Alert[{}] running every {:?}",
                alert.name,
                interval.period()
            );
            // Execute the query and the analysis
            execute_analysis(&query, &alert, &qtype, &pool.get().unwrap());
        }
    });
    // Add information into our AHashMap protected by RwLock (multiple readers, one write at most)
    RUNNING_ALERT.write().unwrap().insert(alert_id, alert_task);
    ALERTS_LIST.write().unwrap().push(calert);
}

fn get_alerts() -> Result<Vec<Alerts>, AppError> {
    let mut alerts = Vec::new();
    let path = CONFIG
        .get_string("ALERTS_PATH")
        .expect("No ALERTS_PATH defined.");

    for entry in WalkDir::new(path).min_depth(1) {
        let entry = entry.unwrap();
        trace!("Creating alert for {}", entry.path().display());

        let content = std::fs::read_to_string(entry.path());
        match content {
            Ok(mut alerts_str) => {
                let alert: Result<Alerts, simd_json::Error> = simd_json::from_str(&mut alerts_str);
                match alert {
                    Ok(al) => alerts.push(al),
                    Err(e) => warn!(
                        "Cannot convert {:?} into an object due to: {:?}",
                        entry.path().display(),
                        e
                    ),
                }
            }
            Err(e) => warn!("Cannot read {:?} due to: {:?}", entry.path().display(), e),
        }
    }

    Ok(alerts)
}

/// Start the monitoring tasks for each alarms
pub fn launch_monitoring(pool: Pool) -> Result<(), AppError> {
    // Get the alerts from the database currently present
    let alerts: Vec<Alerts> = get_alerts()?;

    // Start the alerts monitoring for real
    for alert in alerts {
        launch_alert_task(alert, pool.clone())
    }

    // TODO - React to SIGHUP (reload Alerts list)
    // let signals = Signals::new(&[SIGHUP]).expect("Couldn't register Signal");
    // tokio::spawn(hot_reload_alerts(signals));

    Ok(())
}
