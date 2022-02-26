use super::{analysis::execute_analysis, query::*, CdcChange};
use crate::{utils::CdcKind, ALERTS_CONFIG, ALERTS_CURR_ID, ALERTS_LIST, CONFIG, RUNNING_ALERT};

use futures_util::StreamExt;
use sproot::{
    errors::AppError,
    models::{Alerts, AlertsConfig, Host, HostTargeted},
    Pool,
};
use std::{
    io::{Error, ErrorKind},
    time::Duration,
};
use tokio::net::TcpStream;
use tokio_tungstenite::{connect_async, MaybeTlsStream, WebSocketStream};

use std::sync::atomic::Ordering;

/// Helper method that connect to the WS passed as URL and return the Stream
async fn connect_to_ws(url: &str) -> Result<WebSocketStream<MaybeTlsStream<TcpStream>>, Error> {
    match connect_async(url).await {
        Ok(val) => {
            debug!("Websocket: {} handshake completed", url);
            Ok(val.0)
        }
        Err(err) => {
            error!("Websocket: error while connecting {}: \"{}\"", url, err);
            Err(Error::new(ErrorKind::Other, err.to_string()))
        }
    }
}

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
    // Add the task into our AHashMap protected by RwLock (multiple readers, one write at most)
    RUNNING_ALERT.write().unwrap().insert(alert_id, alert_task);
    // Add the alert into the ALERTS_LIST
    ALERTS_LIST.write().unwrap().push(calert);
}

/// Construct the AlertsConfig that will be launched
fn get_alerts_config() -> Vec<AlertsConfig> {
    let path = CONFIG
        .get_string("ALERTS_PATH")
        .expect("No ALERTS_PATH defined.");

    AlertsConfig::from_configs_path(&path)
}

/// Connect to websocket and execute new alerts
fn launch_websocket(pool: Pool) {
    tokio::spawn(async move {
        let domain = CONFIG
            .get_string("WSS_DOMAIN")
            .expect("Missing WSS_DOMAIN in the config file.");

        // Construct the update_url using domain
        let mut update_url = String::with_capacity(35 + domain.len());
        update_url.push_str("wss://");
        update_url.push_str(&domain);
        update_url.push_str("/ws?query=insert,delete:hosts");
        // Connect to the WS for the * (insert, update, delete) type
        let mut ws_stream = connect_to_ws(&update_url).await.unwrap();

        // While we have some message, read them and wait for the next one
        while let Some(msg) = ws_stream.next().await {
            match msg {
                Err(err) => error!("WebSocket: message is an error: {:?}", err),
                Ok(msg) if msg.is_text() => {
                    // Convert msg into String
                    let mut msg = msg.into_text().expect("Cannot convert message to text");
                    trace!("Websocket: Message received: \"{}\"", msg);
                    // Construct data from str using Serde
                    let data: CdcChange = simd_json::from_str(&mut msg).unwrap();
                    // Get the host_uuid that received the change
                    let host_uuid_idx = data.columnnames.iter().position(|item| item == "uuid");
                    if host_uuid_idx.is_none() {
                        error!("WebSocket: host_uuid is not present in the CdcChange");
                        continue;
                    }
                    let host_uuid = &data.columnnames[host_uuid_idx.unwrap()];
                    // Get the hostname that received the change
                    let hostname_idx = data.columnnames.iter().position(|item| item == "hostname");
                    if hostname_idx.is_none() {
                        error!("WebSocket: hostname is not present in the CdcChange");
                        continue;
                    }
                    let hostname = &data.columnnames[hostname_idx.unwrap()];

                    match data.kind {
                        CdcKind::Delete => {
                            trace!("Websocket: running CdcKind::Delete");
                            // Get the alerts IDs for this hosts (if any)
                            // The READ lock will be held for the whole scope
                            let alerts_list = &*ALERTS_LIST.read().unwrap();
                            let matched: Vec<&Alerts> = alerts_list
                                .iter()
                                .filter(|i| &i.host_uuid == host_uuid)
                                .collect();

                            // For all Alerts that matched the host_uuid
                            for alert in matched {
                                let mut running = RUNNING_ALERT.write().unwrap();
                                // Remove the running task from the Vec
                                let task = running.remove(&alert.id);
                                if let Some(task) = task {
                                    // Abort the running task
                                    task.abort();
                                }
                            }
                            // TODO - Might want to delete the matched from ALERTS_LIST
                        }
                        CdcKind::Insert => {
                            trace!("Websocket: running CdcKind::Insert");
                            // Get the ALERTS_CONFIG (read) and filter those with ALL or SPECIFIC(host_uuid);
                            // The READ lock will be held for the whole scope
                            let alerts_config = &*ALERTS_CONFIG.read().unwrap();
                            let matched_config: Vec<&AlertsConfig> = alerts_config
                                .iter()
                                .filter(|e| match e.host_targeted.as_ref().unwrap() {
                                    HostTargeted::ALL => true,
                                    HostTargeted::SPECIFIC(uuid) => uuid == host_uuid,
                                })
                                .collect();

                            for config in matched_config {
                                // Build the Alerts from the config & hostname & host_uuid
                                let alert = Alerts::build_from_config(
                                    config.to_owned(),
                                    host_uuid.to_owned(),
                                    hostname.to_owned(),
                                    ALERTS_CURR_ID.fetch_add(1, Ordering::Relaxed) as i32,
                                );
                                // Start the analysis
                                launch_alert_task(alert, pool.clone())
                            }
                        }
                        _ => trace!("WebSocket: CdcKind not supported"),
                    }
                }
                _ => trace!("WebSocket: Message was Ok but wasn't a Text"),
            }
        }
    });
}

/// Start the monitoring tasks for each alarms
pub fn launch_monitoring(pool: Pool) -> Result<(), AppError> {
    // Get the AlertsConfig from the ALERTS_PATH folder
    let alerts_config: Vec<AlertsConfig> = get_alerts_config();
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

    // Start a WebSocket listening for new/deleted hosts to set up alerts.
    launch_websocket(pool);

    Ok(())
}
