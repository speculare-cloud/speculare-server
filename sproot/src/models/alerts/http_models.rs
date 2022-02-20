use std::io::ErrorKind;

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Alerts {
    pub id: i32,
    pub name: String,
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
    /// Need to be sure that id, host_uuid and hostname are not None
    pub fn from_xo(xo: AlertsXo) -> Result<Self, std::io::Error> {
        if xo.id.is_none() {
            return Err(std::io::Error::new(ErrorKind::Other, "id cannot be None"));
        }
        if xo.host_uuid.is_none() || xo.hostname.is_none() {
            return Err(std::io::Error::new(
                ErrorKind::Other,
                "host_uuid and/or hostname cannot be None",
            ));
        }

        Ok(Self {
            id: xo.id.unwrap(),
            name: xo.name,
            table: xo.table,
            lookup: xo.lookup,
            timing: xo.timing,
            warn: xo.warn,
            crit: xo.crit,
            info: xo.info,
            host_uuid: xo.host_uuid.unwrap(),
            hostname: xo.hostname.unwrap(),
            where_clause: xo.where_clause,
        })
    }
}

/// Used for folder structure in the ALERT_PATH
/// Hostname & host_uuid can depends on the parent
/// folder.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AlertsXo {
    pub id: Option<i32>,
    pub name: String,
    pub table: String,
    pub lookup: String,
    pub timing: i32,
    pub warn: String,
    pub crit: String,
    pub info: Option<String>,
    pub host_uuid: Option<String>,
    pub hostname: Option<String>,
    pub where_clause: Option<String>,
}
