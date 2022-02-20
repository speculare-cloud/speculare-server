use super::{analysis::execute_analysis, query::*};
use crate::{ALERTS_CURR_ID, ALERTS_LIST, CONFIG, RUNNING_ALERT};

use sproot::{
    errors::AppError,
    models::{Alerts, AlertsXo, Host},
    Pool,
};
use std::sync::atomic::Ordering;
use std::{path::PathBuf, time::Duration};
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

fn get_alerts(pool: &Pool) -> Result<Vec<Alerts>, AppError> {
    let mut alerts = Vec::new();
    let path = CONFIG
        .get_string("ALERTS_PATH")
        .expect("No ALERTS_PATH defined.");

    // TODO - If more than 999 hosts, get them too
    let hosts = Host::list_hosts(&pool.get()?, 999, 0)?;

    for entry in WalkDir::new(&path).min_depth(1).max_depth(2) {
        let mut for_all: bool = false;
        let mut specific_host_uuid: Option<&str> = None;
        let entry = entry.unwrap();

        // Skip if it's a directory
        if entry.path().is_dir() {
            continue;
        }

        // Check if it's for all hosts or not (or specific host)
        if let Some(parent_entry) = entry.path().parent() {
            // TODO - Do we want to force a particular folder for all hosts ?
            if parent_entry == PathBuf::from(&path) {
                for_all = true;
            } else if let Some(parent_name) = parent_entry.file_name() {
                specific_host_uuid = parent_name.to_str();
            }
        }

        trace!(
            "Creating alerts {}; for_all: {}; specific_host_uuid: {:?}",
            entry.path().display(),
            for_all,
            specific_host_uuid
        );

        let content = std::fs::read_to_string(entry.path());
        match content {
            Ok(mut a_str) => {
                let alert: Result<AlertsXo, simd_json::Error> = simd_json::from_str(&mut a_str);
                // Did we correctly transformed the string into a struct?
                match alert {
                    Ok(mut alertxo) => {
                        // TODO - Review this to avoid so much allocations
                        if for_all {
                            // Create as many alerts as needed for each hosts
                            for host in &hosts {
                                let mut alertxo_tmp = &mut alertxo;
                                alertxo_tmp.id =
                                    Some(ALERTS_CURR_ID.fetch_add(1, Ordering::Relaxed) as i32);
                                alertxo_tmp.host_uuid = Some(host.uuid.to_owned());
                                alertxo_tmp.hostname = Some(host.hostname.to_owned());

                                trace!("ID is {:?}", alertxo_tmp.id);
                                if let Ok(alert) = Alerts::from_xo(alertxo_tmp.to_owned()) {
                                    trace!("Created alert {} for {}", alert.name, host.hostname);
                                    alerts.push(alert);
                                } else {
                                    error!("Couldn't transform AlertXo to Alerts: missing host_uuid or hostname");
                                }
                            }
                        } else {
                            // If the alerts is in a specific folder
                            if let Some(parent_folder) = specific_host_uuid {
                                trace!("Parent_folder is defined to {}", parent_folder);
                                let targeted_hosts: Vec<&Host> =
                                    hosts.iter().filter(|h| h.uuid == parent_folder).collect();
                                if targeted_hosts.len() != 1 {
                                    error!("The alert {} targeting {} using folder structure is invalid as host {} does not exists.", alertxo.name, parent_folder, parent_folder);
                                    continue;
                                }
                                let targeted_hosts = targeted_hosts[0];
                                alertxo.host_uuid = Some(targeted_hosts.uuid.to_owned());
                                alertxo.hostname = Some(targeted_hosts.hostname.to_owned());
                            }

                            alertxo.id =
                                Some(ALERTS_CURR_ID.fetch_add(1, Ordering::Relaxed) as i32);

                            trace!("ID is {:?}", alertxo.id);
                            // Convert to alerts and push to the Vec
                            if let Ok(alert) = Alerts::from_xo(alertxo) {
                                trace!("Created alert {} for {}", alert.name, alert.hostname);
                                alerts.push(alert);
                            } else {
                                error!("Couldn't transform AlertXo to Alerts: missing host_uuid or hostname");
                            }
                        }
                    }
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
    let alerts: Vec<Alerts> = get_alerts(&pool)?;

    // Start the alerts monitoring for real
    for alert in alerts {
        launch_alert_task(alert, pool.clone())
    }

    // TODO - Open a Websocket for new hosts (to launch the relevant alerts)
    // TODO - Hot reload of the alerts using SIGHUP (?)
    // let signals = Signals::new(&[SIGHUP]).expect("Couldn't register Signal");
    // tokio::spawn(hot_reload_alerts(signals));

    Ok(())
}
