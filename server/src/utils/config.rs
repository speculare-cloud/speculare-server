use crate::Args;

use clap::Parser;
use config::ConfigError;
use serde::Deserialize;

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
    #[serde(default = "default_https")]
    pub https: bool,
    pub key_priv: Option<String>,
    pub key_cert: Option<String>,
    pub binding: String,
    pub api_token: String,
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

        config_builder.build()?.try_deserialize()
    }
}

fn default_https() -> bool {
    false
}

fn default_maxconn() -> u32 {
    10
}
