use crate::errors::AppError;
use crate::ConnType;

use crate::models::schema::alerts;
use crate::models::schema::alerts::dsl::{_name, alerts as dsl_alerts, host_uuid};

use diesel::*;
use serde::{Deserialize, Serialize};

/// Struct to hold information about alerts
#[derive(Identifiable, Queryable, Debug, Serialize, Deserialize)]
#[table_name = "alerts"]
pub struct Alerts {
    pub id: i64,
    // Name of the alarms (only _ is allowed)
    #[column_name = "_name"]
    pub name: String,
    // Table name targeted
    #[column_name = "_table"]
    pub table: String,
    // Represent the query used to check the alarms against the database's data
    // eg: "average pct 10m of w,x over y,z"
    //     =>(will compute the (10m avg(w)+avg(x) over avg(y)+avg(z)) * 100, result is in percentage as asked using percentage and over)
    // eg: "average abs 10m of x"
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
    // Where SQL condition
    pub where_clause: Option<String>,
}

impl Alerts {
    /// Return a Vector of CpuStats
    /// # Params
    /// * `conn` - The r2d2 connection needed to fetch the data from the db
    /// * `uuid` - The host's uuid we want to get alerts of, this field is optional
    /// * `size` - The number of elements to fetch
    /// * `page` - How many items you want to skip (page * size)
    pub fn get_data(
        conn: &ConnType,
        uuid: Option<&str>,
        size: i64,
        page: i64,
    ) -> Result<Vec<Self>, AppError> {
        let data: Vec<Self> = match uuid {
            Some(val) => dsl_alerts
                .filter(host_uuid.eq(val))
                .limit(size)
                .offset(page * size)
                .order_by(_name.asc())
                .load(conn)?,
            None => dsl_alerts
                .limit(size)
                .offset(page * size)
                .order_by(_name.asc())
                .load(conn)?,
        };

        Ok(data)
    }
}
