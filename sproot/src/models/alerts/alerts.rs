use crate::errors::{AppError, AppErrorType};
use crate::models::schema::alerts;
use crate::models::schema::alerts::dsl::{_name, alerts as dsl_alerts, host_uuid, id};
use crate::ConnType;

use diesel::*;
use serde::{Deserialize, Serialize};

/// Struct to hold information about alerts
#[derive(Identifiable, Queryable, Debug, Serialize, Deserialize, Clone)]
#[table_name = "alerts"]
pub struct Alerts {
    pub id: i32,
    // Name of the alarms (only _ is allowed)
    #[column_name = "_name"]
    pub name: String,
    // Table name targeted
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
    /// Return a Vector of Alerts
    /// # Params
    /// * `conn` - The r2d2 connection needed to fetch the data from the db
    /// * `uuid` - The host's uuid we want to get alerts of, this field is optional
    /// * `size` - The number of elements to fetch
    /// * `page` - How many items you want to skip (page * size)
    pub fn get_list(
        conn: &ConnType,
        uuid: Option<&String>,
        size: i64,
        page: i64,
    ) -> Result<Vec<Self>, AppError> {
        // Depending on if the uuid is present or not
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

    /// Return a single Alerts
    /// # Params
    /// * `conn` - The r2d2 connection needed to fetch the data from the db
    /// * `target_id` - The id of the alerts to get
    pub fn get(conn: &ConnType, target_id: i32) -> Result<Self, AppError> {
        Ok(dsl_alerts.find(target_id).first(conn)?)
    }

    /// Insert a new Alerts inside the database
    /// # Params
    /// * `conn` - The r2d2 connection needed to fetch the data from the db
    /// * `alert` - The AlertsDTO struct containing the new alert information
    pub fn insert(conn: &ConnType, alerts: &[AlertsDTO]) -> Result<(), AppError> {
        insert_into(dsl_alerts).values(alerts).execute(conn)?;
        Ok(())
    }

    /// Remove an Alerts inside the database
    /// # Params
    /// * `conn` - The r2d2 connection needed to fetch the data from the db
    /// * `target_id` - The id of the alerts to delete
    pub fn delete(conn: &ConnType, target_id: i32) -> Result<(), AppError> {
        let res = delete(dsl_alerts.filter(id.eq(target_id))).execute(conn)?;
        if res == 1 {
            Ok(())
        } else {
            Err(AppError {
                message: None,
                cause: None,
                error_type: AppErrorType::NotFound,
            })
        }
    }

    /// Update an Alerts inside the database
    /// # Params
    /// * `conn` - The r2d2 connection needed to fetch the data from the db
    /// * `alert` - The AlertsDTOUpdate struct containing the updated alert information
    /// * `target_id` - The id of the alerts to update
    pub fn update(
        conn: &ConnType,
        alert: &AlertsDTOUpdate,
        target_id: i32,
    ) -> Result<(), AppError> {
        let res = update(dsl_alerts.filter(id.eq(target_id)))
            .set(alert)
            .execute(conn)?;
        if res == 1 {
            Ok(())
        } else {
            Err(AppError {
                message: None,
                cause: None,
                error_type: AppErrorType::NotFound,
            })
        }
    }
}

/// Insertable struct (no id fields => which is auto generated)
#[derive(Insertable, Deserialize, Serialize, Debug)]
#[table_name = "alerts"]
pub struct AlertsDTO {
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
    pub hostname: String,
    pub where_clause: Option<String>,
}

/// Using a specific struct for the Update allow us to pass all as None expect the fields we want to update
#[derive(AsChangeset, Deserialize, Serialize, Debug)]
#[table_name = "alerts"]
pub struct AlertsDTOUpdate {
    #[column_name = "_name"]
    pub name: Option<String>,
    #[column_name = "_table"]
    pub table: Option<String>,
    pub lookup: Option<String>,
    pub timing: Option<i32>,
    pub warn: Option<String>,
    pub crit: Option<String>,
    pub info: Option<String>,
    pub host_uuid: Option<String>,
    pub hostname: Option<String>,
    pub where_clause: Option<String>,
}
