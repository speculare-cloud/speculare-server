use crate::models::schema::{alerts, incidents};

use serde::{Deserialize, Serialize};

// ====================
// Http specific struct
// Meaning those are used whenever
// there is a POST request
// ====================
#[derive(Insertable, AsChangeset, Deserialize, Serialize, Debug)]
#[table_name = "alerts"]
pub struct HttpAlerts {
    #[column_name = "_name"]
    pub name: String,
    #[column_name = "_table"]
    pub table: String,
    pub lookup: String,
    pub timing: i32,
    pub warn: String,
    pub crit: String,
    pub info: Option<String>,
    pub host_uuid: String,
    pub where_clause: Option<String>,
}

#[derive(Insertable, AsChangeset, Deserialize, Serialize, Debug)]
#[table_name = "incidents"]
pub struct HttpIncidents {
    pub result: String,
    pub updated_at: chrono::NaiveDateTime,
    pub host_uuid: String,
    pub status: i32,
    pub alerts_id: i32,
    pub alerts_name: String,
    pub alerts_table: String,
    pub alerts_lookup: String,
    pub alerts_timing: i32,
    pub alerts_warn: String,
    pub alerts_crit: String,
    pub alerts_info: Option<String>,
    pub alerts_where_clause: Option<String>,
}
