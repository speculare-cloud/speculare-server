use serde::{Deserialize, Serialize};

/// Struct to hold information about alerts
#[derive(Serialize, Deserialize, Debug)]
pub struct Alerts {
    // Name of the alarms (only _ is allowed)
    pub name: String,
    // Table name targeted
    pub table: String,
    // Represent the query used to check the alarms against the database's data
    // eg: "average 10m percentage of w,x over y,z"
    //     =>(will compute the (10m avg(w)+avg(x) over avg(y)+avg(z)) * 100, result is in percentage as asked using percentage and over)
    pub lookup: String,
    // Number of seconds between checks
    pub timing: i64,
    // $this > 50 ($this refer to the result of the query, should return a bool)
    pub warn: String,
    // $this > 80 ($this refer to the result of the query, should return a bool)
    pub crit: String,
    // Description of the alarms
    pub info: String,
    // Targeted host
    pub host_uuid: String,
    // Where SQL condition
    pub where_clause: Option<String>,
}
