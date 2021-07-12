use crate::models::schema::alerts;

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
