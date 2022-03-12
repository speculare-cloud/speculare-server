use crate::Args;

use clap::Parser;
use config::ConfigError;
use lettre::message::Mailbox;
use serde::{de, Deserialize, Deserializer};

#[derive(Debug, Clone, Deserialize, PartialEq)]
pub enum AlertSource {
    #[serde(rename = "files")]
    Files,
    #[serde(rename = "database")]
    Database,
}

#[derive(Debug, Deserialize, Clone)]

pub struct Config {
    // GLOBAL SETTINGS
    pub alerts_source: AlertSource,
    #[serde(default = "default_alert_path")]
    pub alerts_path: String,
    pub wss_domain: String,

    // POSTGRESQL CONNECTION
    pub database_url: String,
    #[serde(default = "default_maxconn")]
    pub database_max_connection: u32,

    // SMTP SETTINGS
    #[serde(default = "default_smtp_port")]
    pub smtp_port: u16,
    #[serde(default = "default_smtp_tls")]
    pub smtp_tls: bool,
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

fn default_smtp_port() -> u16 {
    587
}

fn default_smtp_tls() -> bool {
    true
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
