use crate::{
    embedded_migrations,
    utils::{
        config::AlertSource, mail::test_smtp_transport, monitoring::launch_monitoring,
        ws_alerts::listen_alerts_changes, ws_hosts::listen_hosts_changes,
    },
    CONFIG,
};

use sproot::{models::Alerts, Pool};

/// Will start the program normally (launch alerts, ...)
pub async fn flow_run_start(pool: Pool) -> std::io::Result<()> {
    // Check if the SMTP server host is "ok"
    test_smtp_transport();

    // Get a connection from the R2D2 pool
    let pooled_conn = match pool.get() {
        Ok(pooled) => pooled,
        Err(e) => {
            error!(
                "Cannot get a connection from the pool to apply migrations: {}",
                e
            );
            std::process::exit(1);
        }
    };

    // Apply the migrations to the database
    if let Err(e) = embedded_migrations::run(&pooled_conn) {
        error!("Cannot apply the migrations: {}", e);
        std::process::exit(1);
    }

    // If alerts_source is == Files, we should delete all
    // alerts in the database because they'll be recreated.
    // Deleting and recreating allow us to avoid computing the
    // diff between database and actual to remove the old alerts.
    if CONFIG.alerts_source == AlertSource::Files {
        match Alerts::delete_all(&pooled_conn) {
            Ok(_) => {}
            Err(e) => {
                error!("Cannot delete the alerts from the database: {}", e);
                std::process::exit(1);
            }
        };
    }

    // Launch the monitoring of each alarms
    launch_monitoring(pool.clone())
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e.message()))?;

    if CONFIG.alerts_source == AlertSource::Files {
        // Start a WebSocket listening for inserted hosts to set up alerts.
        listen_hosts_changes(pool).await
    } else {
        // Start a WebSocket listening for new/deleted/update alerts.
        listen_alerts_changes(pool).await
    }
}
