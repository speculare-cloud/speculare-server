use sproot::models::AlertsConfig;

use crate::CONFIG;

/// Will check the AlertsConfig syntax for potential errors
pub fn flow_check_start() {
    // Need to get the Alerts
    match AlertsConfig::from_configs_path(&CONFIG.alerts_path) {
        Ok(_) => println!("\nNo issue found, you can restart speculare-alerts."),
        Err(_) => {
            println!("\nFailed to get AlertsConfig, check tbe logs for more info.\n> If you can't see the logs, try settings RUST_LOG to trace in the config.toml");
            std::process::exit(1)
        }
    }
}
