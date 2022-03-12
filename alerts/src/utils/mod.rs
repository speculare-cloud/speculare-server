use diesel::{
    sql_types::{Float8, Timestamp},
    *,
};
use serde::{Deserialize, Serialize};

mod qtype;
pub use qtype::*;

mod notifications;
pub use notifications::*;

pub mod alerts;
pub mod analysis;
pub mod config;
pub mod impls;
pub mod monitoring;
pub mod query;
pub mod websocket;
pub mod ws_alerts;
pub mod ws_hosts;

/// Enum used to hold either i32, String or Option<String> (from CDC)
///
/// Using untagged to give serde the opportinity to try match without a structure.
#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
enum Thing {
    Number(i32),
    String(String),
    OptionString(Option<String>),
}

/// Enum to represente the kind of the CdcChange message
///
/// Convert to lowercase to match with the message "update", "insert"
#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(rename_all = "lowercase")]
enum CdcKind {
    Update,
    Insert,
    Delete,
}

/// Structure holding the info we need from the WebSocket
#[derive(Serialize, Deserialize, Debug)]
struct CdcChange {
    columnnames: Vec<String>,
    columnvalues: Vec<Thing>,
    kind: CdcKind,
    table: String,
}

/// Struct to hold the return from the sql_query for percentage query
#[derive(QueryableByName, Debug)]
pub struct PctDTORaw {
    #[sql_type = "Float8"]
    pub numerator: f64,
    #[sql_type = "Float8"]
    pub divisor: f64,
    #[sql_type = "Timestamp"]
    pub time: chrono::NaiveDateTime,
}

/// Struct to hold the return from the sql_query for absolute query
#[derive(QueryableByName, Debug)]
pub struct AbsDTORaw {
    #[sql_type = "Float8"]
    pub value: f64,
    #[sql_type = "Timestamp"]
    pub time: chrono::NaiveDateTime,
}

/// Constant list of disallowed statement in the SQL query to avoid somthg bad
pub const DISALLOWED_STATEMENT: &[&str] = &[
    "DELETE",
    "UPDATE",
    "INSERT",
    //"CREATE", => conflict with created_at, TODO FIX LATER
    "ALTER",
    "DROP",
    "TRUNCATE",
    "GRANT",
    "REVOKE",
    "BEGIN",
    "COMMIT",
    "SAVEPOINT",
    "ROLLBACK",
];

/// Enum representing the current Status of the Incidents
pub enum IncidentStatus {
    Active,
    Resolved,
}

impl std::fmt::Display for IncidentStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            IncidentStatus::Active => {
                write!(f, "Active")
            }
            IncidentStatus::Resolved => {
                write!(f, "Resolved")
            }
        }
    }
}

impl From<i32> for IncidentStatus {
    fn from(v: i32) -> Self {
        match v {
            0 => IncidentStatus::Active,
            _ => IncidentStatus::Resolved,
        }
    }
}

/// Enum representing the Severity of the Incidents
#[derive(Clone)]
pub enum Severity {
    Warning,
    Critical,
}

impl std::fmt::Display for Severity {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Severity::Warning => {
                write!(f, "Warning")
            }
            Severity::Critical => {
                write!(f, "Critical")
            }
        }
    }
}

impl From<i32> for Severity {
    fn from(v: i32) -> Self {
        match v {
            0 => Severity::Warning,
            _ => Severity::Critical,
        }
    }
}
