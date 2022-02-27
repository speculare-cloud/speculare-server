use crate::Args;

use clap::Parser;
use config::ConfigError;
use lettre::message::Mailbox;
use serde::{de, Deserialize, Deserializer};

#[derive(Debug, Deserialize, Clone)]

pub struct Config {
    // WHERE ARE THE ALERTSCONFIG
    #[serde(default = "default_alert_path")]
    pub alerts_path: String,

    // POSTGRESQL DB CONFIGS
    pub database_url: String,
    #[serde(default = "default_maxconn")]
    pub database_max_connection: u32,

    // HTTP API CONFIGS
    #[serde(default = "default_https")]
    pub https: bool,
    pub key_priv: Option<String>,
    pub key_cert: Option<String>,
    #[serde(default = "default_binding")]
    pub binding: String,
    pub api_token: String,

    // PGCDC INSTANCE'S URL
    pub wss_domain: String,

    // SMTP CREDENTIALS
    pub smtp_host: String,
    pub smtp_user: String,
    pub smtp_password: String,
    #[serde(deserialize_with = "mailbox_deser")]
    pub smtp_email_sender: Mailbox,
    #[serde(deserialize_with = "mailbox_deser")]
    pub smtp_email_receiver: Mailbox,
}

impl Config {
    pub fn new() -> Result<Self, ConfigError> {
        let args = Args::parse();

        let config_builder = config::Config::builder().add_source(config::File::new(
            &args
                .config_path
                .unwrap_or_else(|| "/etc/speculare/alerts.config".to_owned()),
            config::FileFormat::Toml,
        ));

        config_builder.build()?.try_deserialize()
    }
}

fn default_alert_path() -> String {
    String::from("/etc/speculare/alerts-configs")
}

fn default_https() -> bool {
    false
}

fn default_binding() -> String {
    String::from("0.0.0.0:8080")
}

fn default_maxconn() -> u32 {
    10
}

fn mailbox_deser<'de, D>(data: D) -> Result<Mailbox, D::Error>
where
    D: Deserializer<'de>,
{
    let s: String = de::Deserialize::deserialize(data)?;
    match s.parse() {
        Ok(res) => Ok(res),
        Err(e) => Err(std::io::Error::new(
            std::io::ErrorKind::Other,
            format!("Mailbox error for \"{}\": {}", s, e),
        )),
    }
    .map_err(de::Error::custom)
}
