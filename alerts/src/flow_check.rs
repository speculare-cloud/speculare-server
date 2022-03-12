use crate::{
    utils::{
        analysis::execute_analysis, config::AlertSource, mail::test_smtp_transport,
        monitoring::alerts_from_config, query::construct_query,
    },
    ALERTS_CONFIG, CONFIG,
};

use sproot::{
    models::{Alerts, AlertsConfig},
    Pool,
};

/// Will check the AlertsConfig & SMTP syntax for potential errors
pub fn flow_check_start(pool: Pool) {
    // Check if the SMTP server host is "ok"
    test_smtp_transport();

    // If the AlertSource is database, we don't need to check the Alerts
    // as they are created using the API and already checked at creation
    if CONFIG.alerts_source == AlertSource::Database {
        println!("\nEverything went well, no errors found !");
        return;
    }

    // Need to get the Alerts
    let alerts_config = match AlertsConfig::from_configs_path(&CONFIG.alerts_path) {
        Ok(alerts_config) => alerts_config,
        Err(_) => {
            println!("\nFailed to get AlertsConfig, check tbe logs for more info.\n> If you can't see the logs, try settings RUST_LOG to trace in the config.toml");
            std::process::exit(1)
        }
    };

    // New scope: Drop the lock as soon as it's not needed anymore
    {
        // Move the local alerts_config Vec to the global ALERTS_CONFIG
        let mut x = ALERTS_CONFIG.write().unwrap();
        let _ = std::mem::replace(&mut *x, alerts_config);
    }

    let conn = match pool.get() {
        Ok(conn) => conn,
        Err(e) => {
            error!("Cannot get a conn for the alerts_from_config: {}", e);
            std::process::exit(1);
        }
    };

    // Convert the AlertsConfig to alert
    let alerts: Vec<Alerts> = match alerts_from_config(&conn) {
        Ok(alerts) => alerts,
        Err(e) => {
            error!("Failed to launch monitoring: {}", e);
            std::process::exit(1);
        }
    };

    // Dry run for the alerts and exit in case of error
    for alert in alerts {
        let (query, qtype) = match construct_query(&alert) {
            Ok((query, qtype)) => (query, qtype),
            Err(e) => {
                error!(
                    "Alert {} for host_uuid {:.6} cannot build the query: {}",
                    alert.name, alert.host_uuid, e
                );
                std::process::exit(1);
            }
        };
        execute_analysis(&query, &alert, &qtype, &pool.get().unwrap());
    }

    println!("\nEverything went well, no errors found !");
}
