use crate::{ALERTS_CONFIG, ALERTS_CURR_ID, CONFIG};

use sproot::{
    errors::AppError,
    models::{Alerts, AlertsConfig, Host, HostTargeted},
    Pool,
};
use std::sync::atomic::Ordering;

use super::{alerts::start_alert_task, hosts_changes::listen_hosts_changes};

/// Start the monitoring tasks for each alarms
pub fn launch_monitoring(pool: Pool) -> Result<(), AppError> {
    // Get the AlertsConfig from the ALERTS_PATH folder
    let alerts_config: Vec<AlertsConfig> =
        match AlertsConfig::from_configs_path(&CONFIG.alerts_path) {
            Ok(alerts) => alerts,
            Err(_) => std::process::exit(1),
        };
    // New scope: Drop the lock as soon as it's not needed anymore
    {
        // Move the local alerts_config Vec to the global ALERTS_CONFIG
        let mut x = ALERTS_CONFIG.write().unwrap();
        let _ = std::mem::replace(&mut *x, alerts_config);
    }

    // TODO - If more than 50 hosts, get them too (paging).
    let hosts = &Host::list_hosts(&pool.get()?, 50, 0)?;

    let mut alerts: Vec<Alerts> = Vec::new();
    // For each alerts config, create the Alerts corresponding
    // with the host & host_uuid & id defined.
    for aconfig in &*ALERTS_CONFIG.read().unwrap() {
        let cloned_config = aconfig.clone();
        match aconfig.host_targeted.as_ref().unwrap() {
            HostTargeted::SPECIFIC(val) => {
                let thosts: Vec<&Host> = hosts.iter().filter(|h| &h.uuid == val).collect();
                if thosts.len() != 1 {
                    error!(
                        "The host {} in the AlertConfig {} does not exists.",
                        &val, &aconfig.name
                    );
                    continue;
                }
                info!("Created the alert {} for {}", &aconfig.name, thosts[0].uuid);

                alerts.push(Alerts::build_from_config(
                    cloned_config,
                    thosts[0].uuid.to_owned(),
                    thosts[0].hostname.to_owned(),
                    ALERTS_CURR_ID.fetch_add(1, Ordering::Relaxed) as i32,
                ));
            }
            HostTargeted::ALL => {
                for host in hosts {
                    info!("Created the alert {} for {}", &aconfig.name, host.uuid);

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
        start_alert_task(alert, pool.clone())
    }

    // Start a WebSocket listening for new/deleted hosts to set up alerts.
    listen_hosts_changes(pool);

    Ok(())
}
