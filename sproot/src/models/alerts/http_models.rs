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
