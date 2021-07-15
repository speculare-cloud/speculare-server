use super::{analysis::execute_analysis, query::*, CdcChange, CdcKind};
use crate::{ALERTS_LIST, CONFIG};

use futures_util::StreamExt;
use sproot::{errors::AppError, models::Alerts, Pool};
use std::{
    io::{Error, ErrorKind},
    time::Duration,
};
use tokio::net::TcpStream;
use tokio_tungstenite::{connect_async, MaybeTlsStream, WebSocketStream};

/// Helper method that connect to the WS passed as URL and return the Stream
async fn connect_to_ws(url: &str) -> Result<WebSocketStream<MaybeTlsStream<TcpStream>>, Error> {
    match connect_async(url).await {
        Ok(val) => {
            debug!("WS: {} handshake completed", url);
            Ok(val.0)
        }
        Err(err) => {
            error!("WS: error while connecting {}: \"{}\"", url, err);
            Err(Error::new(ErrorKind::Other, err.to_string()))
        }
    }
}

/// Create the task for a particular alert and add it to the ALERTS_LIST.
fn launch_alert_task(alert: Alerts, pool: Pool) {
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
                    alert.id, statement
                );
                return;
            }
        }

        // Start the real "forever" loop
        loop {
            // Wait for the next tick of our interval
            interval.tick().await;
            trace!("{}: Run every {:?}", alert.name, interval.period());
            // Execute the query and the analysis
            execute_analysis(&query, &alert, &qtype, &pool.get().unwrap());
        }
    });
    // Add information into our HashMap protected by RwLock (multiple readers, one write at most)
    ALERTS_LIST.write().unwrap().insert(alert_id, alert_task);
}

/// Connect to websocket and dispatch task
fn launch_websocket(pool: Pool) {
    tokio::spawn(async move {
        let domain = CONFIG
            .get_str("WSS_DOMAIN")
            .expect("Missing WSS_DOMAIN in the config file.");
        // Connect to the WS for the update type
        let update_url = format!("wss://{}/ws?query=update:alerts", domain);
        let mut ws_update = connect_to_ws(&update_url).await.unwrap();

        // Connect to the WS for the insert type
        let insert_url = format!("wss://{}/ws?query=update:alerts", domain);
        let mut ws_insert = connect_to_ws(&insert_url).await.unwrap();

        // While we have some message, read them and wait for the next one
        // We also combine both stream into "one", this is not really true but
        // we do poll both of them using tokio::select! macro.
        while let Some(msg) = tokio::select! {
            v = ws_update.next() => v,
            v = ws_insert.next() => v,
        } {
            if let Ok(msg) = msg {
                // Assert that msg is text (should always be as it's JSON)
                if !msg.is_text() {
                    continue;
                }
                // Convert msg into String
                let mut msg = msg.into_text().expect("Cannot convert message to text");
                trace!("WS: Message received: \"{}\"", msg);
                // Construct data from str using Serde
                let data: CdcChange = simd_json::from_str(&mut msg).unwrap();
                // Construct alert from CdcChange (using columnname and columnvalues)
                let alert: Result<Alerts, Error> = (&data).into();
                // Check if alert is an error (happen if not all fields are presents)
                if alert.is_err() {
                    error!("Cannot build Alerts from CdcChange: {}", alert.unwrap_err());
                    continue;
                }
                // Unwrap as we checked before
                let alert = alert.unwrap();
                // If the kind is Update, we might need to shutdown the previous task
                if data.kind == CdcKind::Update {
                    // Get the HashMap from the RwLock
                    let running = ALERTS_LIST.read().unwrap();
                    // Get the task using the alert's id
                    let task = running.get(&alert.id);
                    // If exist, abort the task
                    if let Some(task) = task {
                        task.abort();
                    }
                }
                // TODO - In the future when pausing an alert is implemented
                // this is where we should make the check.
                launch_alert_task(alert, pool.clone());
            }
        }
    });
}

/// Start the monitoring tasks for each alarms
pub fn launch_monitoring(pool: Pool) -> Result<(), AppError> {
    // Get the alerts from the database currently present
    let alerts: Vec<Alerts> = Alerts::get_list(&pool.get()?, None, 9999, 0)?;

    // Foreach alerts
    for alert in alerts {
        // Call the function responsible for the creation of the task
        launch_alert_task(alert, pool.clone())
    }

    // Create a WS client that will connect to the Websocket of PGCDC about Alerts
    // This client will abort a task of an Alarm that is being updated and restart it
    // and will also create new task for new Alarms that are just being created after the startup.
    launch_websocket(pool);

    Ok(())
}
