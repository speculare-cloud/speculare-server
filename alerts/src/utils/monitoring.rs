use crate::{ALERTS_CONFIG, CONFIG};

use super::{alerts::start_alert_task, config::AlertSource};

use sproot::{
    errors::AppError,
    models::{Alerts, AlertsConfig, Host, HostTargeted},
    ConnType, Pool,
};

pub fn alerts_from_config(conn: &ConnType) -> Result<Vec<Alerts>, AppError> {
    // TODO - If more than 50 hosts, get them too (paging).
    let hosts = &Host::list_hosts(conn, 50, 0)?;

    let mut alerts: Vec<Alerts> = Vec::new();
    // For each alerts config, create the Alerts corresponding
    // with the host & host_uuid & id defined.
    for aconfig in &*ALERTS_CONFIG.read().unwrap() {
        let cloned_config = aconfig.clone();
        match aconfig.host_targeted.as_ref().unwrap() {
            HostTargeted::SPECIFIC(val) => {
                let thosts: Vec<&Host> = hosts.iter().filter(|h| &h.uuid == val).collect();
                if thosts.len() != 1 {
                    return Err(AppError {
                        message: Some(format!(
                            "The host {} in the AlertConfig {} does not exists.",
                            &val, &aconfig.name
                        )),
                        cause: None,
                        error_type: sproot::errors::AppErrorType::NotFound,
                    });
                }
                let id = Alerts::generate_id_from(&thosts[0].uuid, &aconfig.name);

                info!(
                    "Created the alert {} for {:.6} with id {}",
                    &aconfig.name, thosts[0].uuid, id
                );

                alerts.push(Alerts::build_from_config(
                    cloned_config,
                    thosts[0].uuid.to_owned(),
                    thosts[0].hostname.to_owned(),
                    id,
                ));
            }
            HostTargeted::ALL => {
                for host in hosts {
                    let id = Alerts::generate_id_from(&host.uuid, &aconfig.name);

                    info!(
                        "Created the alert {} for {:.6} with id {}",
                        &aconfig.name, host.uuid, id
                    );

                    alerts.push(Alerts::build_from_config(
                        cloned_config.clone(),
                        host.uuid.to_owned(),
                        host.hostname.to_owned(),
                        id,
                    ));
                }
            }
        }
    }

    Ok(alerts)
}

fn alerts_from_files(conn: &ConnType) -> Result<Vec<Alerts>, AppError> {
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

    // Convert the AlertsConfig to alerts
    alerts_from_config(conn)
}

fn alerts_from_database(conn: &ConnType) -> Result<Vec<Alerts>, AppError> {
    // Get the alerts from the database
    Alerts::get_list(conn)
}

/// Start the monitoring tasks for each alarms
pub fn launch_monitoring(pool: Pool) -> Result<(), AppError> {
    let conn = &pool.get()?;
    let alerts = match if CONFIG.alerts_source == AlertSource::Files {
        alerts_from_files(conn)
    } else {
        alerts_from_database(conn)
    } {
        Ok(alerts) => alerts,
        Err(e) => {
            error!("Cannot get the alerts: {}", e);
            std::process::exit(1);
        }
    };

    // Start the alerts monitoring for real
    for alert in alerts {
        start_alert_task(alert, pool.clone())
    }

    Ok(())
}
