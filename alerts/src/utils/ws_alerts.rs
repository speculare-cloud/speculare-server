use crate::{
    utils::{alerts::start_alert_task, CdcKind},
    CONFIG, RUNNING_ALERT,
};

use super::{websocket::connect_to_ws, CdcChange};

use futures::StreamExt;
use sproot::{models::Alerts, Pool};

/// Connect to websocket and execute new alerts
pub async fn listen_alerts_changes(pool: Pool) -> std::io::Result<()> {
    let domain = &CONFIG.wss_domain;
    // Construct the update_url using domain
    let mut update_url = String::with_capacity(24 + domain.len());
    update_url.push_str("wss://");
    update_url.push_str(domain);
    update_url.push_str("/ws?query=*:alerts");
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

                // Construct alert from CdcChange (using columnname and columnvalues)
                let alert: Alerts = match (&data).into() {
                    Ok(alert) => alert,
                    Err(e) => {
                        error!(
                            "Cannot construct the alert with the data from the WS: {}",
                            e
                        );
                        continue;
                    }
                };

                match data.kind {
                    CdcKind::Insert => {
                        info!("Websocket: running CdcKind::Insert");
                        start_alert_task(alert, pool.clone())
                    }
                    CdcKind::Update | CdcKind::Delete => {
                        info!("Websocket: running CdcKind::Update or CdcKind::Delete");
                        {
                            // Stop the task's "thread" in it's own scope to drop the lock asap
                            let mut running = RUNNING_ALERT.write().unwrap();
                            let task = running.remove(&alert.id);
                            if let Some(task) = task {
                                task.abort();
                            }
                        }

                        // If it's an Update, start the task again
                        if data.kind == CdcKind::Update {
                            start_alert_task(alert, pool.clone());
                        }
                    }
                }
            }
            _ => {}
        }
    }

    Ok(())
}
