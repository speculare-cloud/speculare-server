use crate::errors::AppError;
use crate::models::schema::alerts;
use crate::models::schema::alerts::dsl::{_name, alerts as dsl_alerts, host_uuid, id};
use crate::ConnType;

use diesel::*;
use log::error;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use walkdir::WalkDir;

#[derive(Identifiable, Insertable, Queryable, Debug, Serialize, Deserialize, Clone)]
#[table_name = "alerts"]
pub struct Alerts {
    // The id is the name + host_uuid
    pub id: String,
    // The name can't be updated as it's used for the id
    #[column_name = "_name"]
    pub name: String,
    #[column_name = "_table"]
    pub table: String,
    // Represent the query used to check the alarms against the database's data
    // eg: "avg pct 10m of w,x over y,z"
    //     =>(will compute the (10m avg(w)+avg(x) over avg(y)+avg(z)) * 100, result is in percentage as asked using percentage and over)
    // eg: "avg abs 10m of x"
    //     =>(will compute based on only an absolute value (no division))
    pub lookup: String,
    // Number of seconds between checks
    pub timing: i32,
    // $this > 50 ($this refer to the result of the query, should return a bool)
    pub warn: String,
    // $this > 80 ($this refer to the result of the query, should return a bool)
    pub crit: String,
    // Description of the alarms
    pub info: Option<String>,
    // Targeted host
    pub host_uuid: String,
    // Targeted hostname
    pub hostname: String,
    // Where SQL condition
    pub where_clause: Option<String>,
}

impl Alerts {
    pub fn generate_id_from(uuid: &str, name: &str) -> String {
        sha1_smol::Sha1::from([uuid.as_bytes(), name.as_bytes()].concat()).hexdigest()
    }

    /// Build from a AlertsConfig, host_uuid, hostname and an id.
    pub fn build_from_config(
        config: AlertsConfig,
        uuid: String,
        hostname: String,
        alert_id: String,
    ) -> Alerts {
        Alerts {
            id: alert_id,
            name: config.name,
            table: config.table,
            lookup: config.lookup,
            timing: config.timing,
            warn: config.warn,
            crit: config.crit,
            info: config.info,
            host_uuid: uuid,
            hostname,
            where_clause: config.where_clause,
        }
    }

    ///
    pub fn get_list(conn: &ConnType) -> Result<Vec<Self>, AppError> {
        Ok(dsl_alerts.order_by(_name.asc()).load(conn)?)
    }

    ///
    pub fn get_list_host(
        conn: &ConnType,
        uuid: &str,
        size: i64,
        page: i64,
    ) -> Result<Vec<Self>, AppError> {
        Ok(dsl_alerts
            .filter(host_uuid.eq(uuid))
            .limit(size)
            .offset(page * size)
            .order_by(_name.asc())
            .load(conn)?)
    }

    ///
    pub fn get(conn: &ConnType, target_id: &str) -> Result<Self, AppError> {
        Ok(dsl_alerts.find(target_id).first(conn)?)
    }

    ///
    pub fn insert(conn: &ConnType, alerts: &[Alerts]) -> Result<usize, AppError> {
        Ok(insert_into(dsl_alerts).values(alerts).execute(conn)?)
    }

    ///
    pub fn ginsert(conn: &ConnType, alerts: &[Alerts]) -> Result<Self, AppError> {
        Ok(insert_into(dsl_alerts).values(alerts).get_result(conn)?)
    }

    ///
    pub fn delete(conn: &ConnType, target_id: &str) -> Result<usize, AppError> {
        Ok(delete(dsl_alerts.filter(id.eq(target_id))).execute(conn)?)
    }

    ///
    pub fn delete_all(conn: &ConnType) -> Result<usize, AppError> {
        Ok(delete(dsl_alerts).execute(conn)?)
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum HostTargeted {
    ALL,
    SPECIFIC(String),
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AlertsConfig {
    pub name: String,
    pub table: String,
    pub lookup: String,
    pub timing: i32,
    pub warn: String,
    pub crit: String,
    pub info: Option<String>,
    pub where_clause: Option<String>,
    pub host_targeted: Option<HostTargeted>,
}

impl AlertsConfig {
    /// Construct AlertsConfig Vec from the path of configs's folder & sub
    #[allow(clippy::result_unit_err)]
    pub fn from_configs_path(path: &str) -> Result<Vec<AlertsConfig>, ()> {
        let mut alerts: Vec<AlertsConfig> = Vec::new();

        for entry in WalkDir::new(&path).min_depth(1).max_depth(2) {
            // Detect if the WalkDir failed to read the folder (permissions/...)
            if entry.is_err() {
                error!("Cannot read the entry due to: {}", entry.unwrap_err());
                return Err(());
            }
            let entry = entry.unwrap();

            // Skip if folder
            if entry.path().is_dir() {
                continue;
            }

            // TODO - Small rewrite needed ? Too much of a stair
            // Get the parent folder name and determine which hosts is targeted
            let host_targeted: HostTargeted = if let Some(parent_entry) = entry.path().parent() {
                if parent_entry == PathBuf::from(&path) {
                    HostTargeted::ALL
                } else if let Some(parent_name) = parent_entry.file_name() {
                    if let Some(targeted_str) = parent_name.to_str() {
                        HostTargeted::SPECIFIC(targeted_str.to_owned())
                    } else {
                        error!("Cannot get the targeted_str, OsStr to Str is None");
                        return Err(());
                    }
                } else {
                    error!("Cannot get the parent_name, parent_entry filename is None");
                    return Err(());
                }
            } else {
                error!("Cannot get the parent_entry directory name, returned None");
                return Err(());
            };

            trace!(
                "Alerts {:?}; HostTargeted[{:?}]",
                entry.path().file_name(),
                host_targeted,
            );

            // Read and store the content of the config into a string
            let content = std::fs::read_to_string(entry.path());
            if content.is_err() {
                error!(
                    "Cannot read {:?}: {}",
                    entry.path().file_name(),
                    content.unwrap_err()
                );
                return Err(());
            }

            // Deserialize the string's config into the struct of AlertsConfig
            let alert_config = simd_json::from_str::<AlertsConfig>(&mut content.unwrap());
            if alert_config.is_err() {
                error!(
                    "Cannot convert {:?} to AlertsConfig: {}",
                    entry.path().file_name(),
                    alert_config.unwrap_err()
                );
                return Err(());
            }
            let mut alert_config = alert_config.unwrap();
            alert_config.host_targeted = Some(host_targeted);

            // Add the AlertsConfig into the Vec
            alerts.push(alert_config);
        }

        Ok(alerts)
    }
}
