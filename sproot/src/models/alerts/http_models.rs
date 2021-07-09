use serde::{Deserialize, Serialize};

// ====================
// Http specific struct
// Meaning those are used whenever
// there is a POST request
// ====================
#[derive(Debug, Serialize, Deserialize)]
pub struct HttpPostAlert {
    pub name: String,
    pub table: String,
    pub lookup: String,
    pub timing: i32,
    pub warn: String,
    pub crit: String,
    pub info: Option<String>,
    pub host_uuid: String,
    pub where_clause: Option<String>,
}
