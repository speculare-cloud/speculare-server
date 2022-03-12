use crate::{
    utils::{alerts::start_alert_task, CdcKind},
    ALERTS_CONFIG, CONFIG,
};

use super::{websocket::connect_to_ws, CdcChange};

use futures::StreamExt;
use sproot::{
    models::{Alerts, AlertsConfig, HostTargeted},
    Pool,
};

/// Connect to websocket and execute new alerts
pub async fn listen_hosts_changes(pool: Pool) -> std::io::Result<()> {
    let domain = &CONFIG.wss_domain;
    // Construct the update_url using domain
    let mut update_url = String::with_capacity(35 + domain.len());
    update_url.push_str("wss://");
    update_url.push_str(domain);
    update_url.push_str("/ws?query=insert:hosts");
    // Connect to the WS for the * (insert, update, delete) type
    let mut ws_stream = connect_to_ws(&update_url).await;

    // While we have some message, read them and wait for the next one
    while let Some(msg) = ws_stream.next().await {
        match msg {
            Err(err) => error!("WebSocket: message is an error: {}", err),
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
                    CdcKind::Insert => {
                        info!("Websocket: running CdcKind::Insert for {:.6}", host_uuid);
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
                                format!("{}_{}", &host_uuid, &hostname),
                            );
                            // Start the analysis
                            start_alert_task(alert, pool.clone())
                        }
                    }
                    _ => trace!("WebSocket: CdcKind not supported"),
                }
            }
            _ => {}
        }
    }

    Ok(())
}
