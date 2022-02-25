use super::{analysis::execute_analysis, query::*};
use crate::{ALERTS_CURR_ID, ALERTS_LIST, CONFIG, RUNNING_ALERT};

use sproot::{
    errors::AppError,
    models::{Alerts, AlertsConfig, Host, HostTargeted},
    Pool,
};

use std::{sync::atomic::Ordering, time::Duration};

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

/// Construct the AlertsConfig that will be launched
fn get_alerts_config() -> Vec<AlertsConfig> {
    let path = CONFIG
        .get_string("ALERTS_PATH")
        .expect("No ALERTS_PATH defined.");

    AlertsConfig::from_configs_path(&path)
}

/// Start the monitoring tasks for each alarms
pub fn launch_monitoring(pool: Pool) -> Result<(), AppError> {
    // Get the AlertsConfig from the ALERTS_PATH folder
    let alerts_config: Vec<AlertsConfig> = get_alerts_config();

    // TODO - If more than 50 hosts, get them too (paging).
    let hosts = &Host::list_hosts(&pool.get()?, 50, 0)?;

    let mut alerts: Vec<Alerts> = Vec::new();
    // For each alerts config, create the Alerts corresponding
    // with the host & host_uuid & id defined.
    for aconfig in alerts_config {
        let cloned_config = aconfig.clone();
        match aconfig.host_targeted.unwrap() {
            HostTargeted::SPECIFIC(val) => {
                let thosts: Vec<&Host> = hosts.iter().filter(|h| h.uuid == val).collect();
                if thosts.len() != 1 {
                    error!(
                        "The host {} in the AlertConfig {} does not exists.",
                        &val, &aconfig.name
                    );
                    continue;
                }

                trace!("Created the alert {} for {}", &aconfig.name, thosts[0].uuid);

                alerts.push(Alerts::build_from_config(
                    cloned_config,
                    thosts[0].uuid.to_owned(),
                    thosts[0].hostname.to_owned(),
                    ALERTS_CURR_ID.fetch_add(1, Ordering::Relaxed) as i32,
                ));
            }
            HostTargeted::ALL => {
                for host in hosts {
                    trace!("Created the alert {} for {}", &aconfig.name, host.uuid);

                    alerts.push(Alerts::build_from_config(
                        cloned_config.clone(),
                        host.uuid.to_owned(),
                        host.hostname.to_owned(),
                        ALERTS_CURR_ID.fetch_add(1, Ordering::Relaxed) as i32,
                    ));
                }
            }
        }
    }

    // Start the alerts monitoring for real
    for alert in alerts {
        launch_alert_task(alert, pool.clone())
    }

    // Start a WebSocket listening for new hosts to set up alerts

    Ok(())
}
