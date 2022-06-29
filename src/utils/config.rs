use crate::Args;

use clap::Parser;
use config::ConfigError;
use serde::Deserialize;
use sproot::models::{default_alertssource, AlertSource};

#[derive(Debug, Deserialize, Clone)]

pub struct Config {
    // POSTGRESQL CONNECTION
    pub database_url: String,
    #[serde(default = "default_maxconn")]
    pub database_max_connection: u32,

    // AUTH POSTGRESQL CONNECTION
    #[cfg(feature = "auth")]
    pub auth_database_url: String,
    #[cfg(feature = "auth")]
    #[serde(default = "default_maxconn")]
    pub auth_database_max_connection: u32,

    // API SETTINGS
    pub binding: String,
    #[serde(default = "default_workers")]
    pub workers: usize,

    // API SECURITY SETTINGS
    #[serde(default = "default_https")]
    pub https: bool,
    pub key_priv: Option<String>,
    pub key_cert: Option<String>,

    #[cfg(not(feature = "auth"))]
    pub api_token: String,

    #[cfg(feature = "auth")]
    pub berta_name: String,
    #[cfg(feature = "auth")]
    pub cookie_secret: String,
    #[cfg(feature = "auth")]
    pub cookie_domain: Option<String>,

    // ALERTS CONFIG
    #[serde(default = "default_alertssource")]
    pub alerts_source: AlertSource,
}

impl Config {
    pub fn new() -> Result<Self, ConfigError> {
        let args = Args::parse();

        let config_builder = config::Config::builder().add_source(config::File::new(
            &args
                .config_path
                .unwrap_or_else(|| "/etc/speculare/server.config".to_owned()),
            config::FileFormat::Toml,
        ));

        let config: Result<Self, ConfigError> = config_builder.build()?.try_deserialize();

        // Assert that the config is correct
        if let Ok(ref config) = config {
            if config.https && (config.key_priv.is_none() || config.key_cert.is_none()) {
                error!(
                    "error: config: 'https' is true but no 'key_priv' and/or 'key_cert' defined"
                );
                std::process::exit(1);
            }
        }

        config
    }
}

fn default_https() -> bool {
    false
}

fn default_maxconn() -> u32 {
    10
}

fn default_workers() -> usize {
    match sys_metrics::cpu::get_logical_count() {
        Ok(count) => count as usize,
        Err(e) => {
            error!(
                "Workers: failed to get the number of workers automatically, defaulting to 4: {}",
                e
            );
            4
        }
    }
}
