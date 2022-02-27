use sproot::models::AlertsConfig;

use crate::CONFIG;

pub fn check_alerts_syntax() {
    let path = CONFIG
        .get_string("ALERTS_PATH")
        .expect("No ALERTS_PATH defined.");
    // Need to get the Alerts
    match AlertsConfig::from_configs_path(&path) {
        Ok(_) => println!("No issue found, you can restart speculare-alerts."),
        Err(_) => {
            println!("Failed to get AlertsConfig, check tbe logs for more info.\n> If you can't see the logs, try settings RUST_LOG to trace in the config.toml");
            std::process::exit(1)
        }
    }
}
