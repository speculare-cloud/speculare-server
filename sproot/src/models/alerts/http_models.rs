use std::{
    io::ErrorKind,
    path::PathBuf,
    sync::{
        atomic::{AtomicUsize, Ordering},
        Arc,
    },
};

use log::error;
use serde::{Deserialize, Serialize};
use walkdir::WalkDir;

use crate::models::Host;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Alerts {
    pub id: i32,
    pub name: String,
    pub table: String,
    // Represent the query used to check the alarms against the database's data
    // eg: "avg pct 10m of w,x over y,z"
    //     =>(will compute the (10m avg(w)+avg(x) over avg(y)+avg(z)) * 100, result is in percentage as asked using percentage and over)
    // eg: "avg abs 10m of x"
    //     =>(will compute based on only an absolute value (no division))
    pub lookup: String,
    // Number of seconds between checks
    pub timing: i32,
    // $this > 50 ($this refer to the result of the query, should return a bool)
    pub warn: String,
    // $this > 80 ($this refer to the result of the query, should return a bool)
    pub crit: String,
    // Description of the alarms
    pub info: Option<String>,
    // Targeted host
    pub host_uuid: String,
    // Targeted hostname
    pub hostname: String,
    // Where SQL condition
    pub where_clause: Option<String>,
}

impl Alerts {
    /// Need to be sure that id, host_uuid and hostname are not None
    pub fn from_xo(xo: AlertsXo) -> Result<Self, std::io::Error> {
        if xo.id.is_none() {
            return Err(std::io::Error::new(ErrorKind::Other, "id cannot be None"));
        }
        if xo.host_uuid.is_none() || xo.hostname.is_none() {
            return Err(std::io::Error::new(
                ErrorKind::Other,
                "host_uuid and/or hostname cannot be None",
            ));
        }

        Ok(Self {
            id: xo.id.unwrap(),
            name: xo.name,
            table: xo.table,
            lookup: xo.lookup,
            timing: xo.timing,
            warn: xo.warn,
            crit: xo.crit,
            info: xo.info,
            host_uuid: xo.host_uuid.unwrap(),
            hostname: xo.hostname.unwrap(),
            where_clause: xo.where_clause,
        })
    }

    pub fn fetch_from_folder(path: &str, hosts: &[Host], counter: Arc<AtomicUsize>) -> Vec<Alerts> {
        let mut alerts: Vec<Alerts> = Vec::new();

        for entry in WalkDir::new(&path).min_depth(1).max_depth(2) {
            // Detect if the WalkDir failed to read the folder (permissions/...)
            let entry = if let Ok(entry) = entry {
                entry
            } else {
                error!("Cannot read the entry due to: {:?}", entry.err());
                continue;
            };
            let mut for_all_hosts = false;
            let mut specific_host = None;
            // For each files/folders in the path directory (alerts's folder)
            // We'll perform the following:
            // - Skip if it's a folder
            // - Get the parent folder name and determine if it's for all hosts
            //   or for a specific host
            // - Building the alert and adding it to the alerts Vec

            // Skip if folder
            if entry.path().is_dir() {
                continue;
            }

            // Get the parent folder name and determine which hosts is targeted
            if let Some(parent_entry) = entry.path().parent() {
                if parent_entry == PathBuf::from(&path) {
                    for_all_hosts = true;
                } else if let Some(parent_name) = parent_entry.file_name() {
                    specific_host = parent_name.to_str();
                }
            }

            trace!(
                "Alerts {:?}; for_all_hosts[{}]; specific_host[{:?}]",
                entry.path().file_name(),
                for_all_hosts,
                specific_host
            );

            let content = std::fs::read_to_string(entry.path());
            if content.is_err() {
                error!(
                    "Cannot read {:?}: {:?}",
                    entry.path().file_name(),
                    content.err()
                );
                continue;
            }

            let alert = simd_json::from_str::<AlertsXo>(&mut content.unwrap());
            if alert.is_err() {
                error!(
                    "Cannot convert {:?} to AlertsXo: {:?}",
                    entry.path().file_name(),
                    alert.err()
                );
                continue;
            }
            let mut alert = alert.unwrap();
            if for_all_hosts {
                for host in hosts {
                    let alert = Self::from_xo_counter(
                        &mut alert,
                        &counter,
                        host.hostname.to_owned(),
                        host.uuid.to_owned(),
                    );

                    trace!("Created alert {} for {}", alert.name, alert.hostname);
                    alerts.push(alert);
                }
            } else {
                if let Some(parent_folder) = specific_host {
                    let targeted_hosts: Vec<&Host> =
                        hosts.iter().filter(|h| h.uuid == parent_folder).collect();
                    if targeted_hosts.len() != 1 {
                        error!("The alert {} targeting {} using folder structure is invalid as host {} does not exists.", alert.name, parent_folder, parent_folder);
                        continue;
                    }
                    let targeted_hosts = targeted_hosts[0];
                    alert.host_uuid = Some(targeted_hosts.uuid.to_owned());
                    alert.hostname = Some(targeted_hosts.hostname.to_owned());
                } else if alert.hostname.is_none() || alert.host_uuid.is_none() {
                    error!("The alert {} is invalid as it does not have a hostname/host_uuid and is not in a 'host' folder.", alert.name);
                    continue;
                }

                alert.id = Some(counter.fetch_add(1, Ordering::Relaxed) as i32);

                trace!("Created alert {} for {:?}", alert.name, alert.hostname);
                alerts.push(alert.into());
            }
        }

        alerts
    }

    fn from_xo_counter(
        alertsxo: &mut AlertsXo,
        counter: &Arc<AtomicUsize>,
        hostname: String,
        host_uuid: String,
    ) -> Self {
        alertsxo.id = Some(counter.fetch_add(1, Ordering::Relaxed) as i32);
        alertsxo.host_uuid = Some(host_uuid);
        alertsxo.hostname = Some(hostname);
        alertsxo.into()
    }
}

/// Need to be sure that id, host_uuid and hostname are defined.
impl From<&mut AlertsXo> for Alerts {
    fn from(alertsxo: &mut AlertsXo) -> Self {
        assert!(alertsxo.id.is_some());
        assert!(alertsxo.host_uuid.is_some());
        assert!(alertsxo.hostname.is_some());

        let alertsxo = alertsxo.to_owned();
        Self {
            id: alertsxo.id.unwrap(),
            name: alertsxo.name,
            table: alertsxo.table,
            lookup: alertsxo.lookup,
            timing: alertsxo.timing,
            warn: alertsxo.warn,
            crit: alertsxo.crit,
            info: alertsxo.info,
            host_uuid: alertsxo.host_uuid.unwrap(),
            hostname: alertsxo.hostname.unwrap(),
            where_clause: alertsxo.where_clause,
        }
    }
}

/// Need to be sure that id, host_uuid and hostname are defined.
impl From<AlertsXo> for Alerts {
    fn from(alertsxo: AlertsXo) -> Self {
        assert!(alertsxo.id.is_some());
        assert!(alertsxo.host_uuid.is_some());
        assert!(alertsxo.hostname.is_some());

        Self {
            id: alertsxo.id.unwrap(),
            name: alertsxo.name,
            table: alertsxo.table,
            lookup: alertsxo.lookup,
            timing: alertsxo.timing,
            warn: alertsxo.warn,
            crit: alertsxo.crit,
            info: alertsxo.info,
            host_uuid: alertsxo.host_uuid.unwrap(),
            hostname: alertsxo.hostname.unwrap(),
            where_clause: alertsxo.where_clause,
        }
    }
}

/// Used for folder structure in the ALERT_PATH
/// Hostname & host_uuid can depends on the parent
/// folder.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AlertsXo {
    pub id: Option<i32>,
    pub name: String,
    pub table: String,
    pub lookup: String,
    pub timing: i32,
    pub warn: String,
    pub crit: String,
    pub info: Option<String>,
    pub host_uuid: Option<String>,
    pub hostname: Option<String>,
    pub where_clause: Option<String>,
}
