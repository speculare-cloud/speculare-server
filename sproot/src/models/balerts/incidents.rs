use crate::errors::{AppError, AppErrorType};
use crate::models::schema::incidents;
use crate::models::schema::incidents::dsl::{
    alerts_id, alerts_name, host_uuid, id, incidents as dsl_incidents, status, updated_at,
};
use crate::ConnType;

use diesel::*;
use serde::{Deserialize, Serialize};

/// Struct to hold information about incidents
/// Yes it's a lot of duplicate from the Alerts but as the Alerts can be updated
/// we need to store a snapshot of the configuration of the said alerts at the
/// time the incidents was created.
#[derive(Identifiable, Queryable, Debug, Serialize, Deserialize, Clone)]
#[table_name = "incidents"]
pub struct Incidents {
    pub id: i32,
    pub result: String,
    pub started_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
    pub resolved_at: Option<chrono::NaiveDateTime>,
    pub host_uuid: String,
    pub hostname: String,
    pub status: i32,
    pub severity: i32,
    pub alerts_id: String,
    pub alerts_name: String,
    pub alerts_table: String,
    pub alerts_lookup: String,
    pub alerts_warn: String,
    pub alerts_crit: String,
    pub alerts_info: Option<String>,
    pub alerts_where_clause: Option<String>,
}

impl Incidents {
    /// Return a Vector of Incidents
    /// # Params
    /// * `conn` - The r2d2 connection needed to fetch the data from the db
    /// * `uuid` - The host's uuid we want to get incidents of, this field is optional
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
            Some(val) => dsl_incidents
                .filter(host_uuid.eq(val))
                .limit(size)
                .offset(page * size)
                .order_by(updated_at.desc())
                .load(conn)?,
            None => dsl_incidents
                .limit(size)
                .offset(page * size)
                .order_by(updated_at.desc())
                .load(conn)?,
        };

        Ok(data)
    }

    /// Determine if the incidents for that specific alert exist and is currently active.
    /// If one is found, return it, otherwise return a Err(NotFound).
    /// # Params
    /// * `conn` - The r2d2 connection needed to fetch the data from the db
    /// * `alert_id` - The id of the alert related to the incident
    pub fn exist(conn: &ConnType, alert_id: String) -> Result<Self, diesel::result::Error> {
        dsl_incidents
            .filter(alerts_id.eq(alert_id).and(status.eq(0)))
            .first(conn)
    }

    /// Determine if the incidents for that specific alert exist and is currently active.
    /// If one is found, return it, otherwise return a Err(NotFound).
    /// # Params
    /// * `conn` - The r2d2 connection needed to fetch the data from the db
    /// * `aname` - The name of the alert related to the incident
    /// * `huuid` - The uuid of the host targeted by the alert related to the incident
    pub fn exist_name(
        conn: &ConnType,
        huuid: &str,
        aname: &str,
    ) -> Result<Self, diesel::result::Error> {
        dsl_incidents
            .filter(
                alerts_name
                    .eq(aname)
                    .and(host_uuid.eq(huuid))
                    .and(status.eq(0)),
            )
            .first(conn)
    }

    /// Return a single Incidents
    /// # Params
    /// * `conn` - The r2d2 connection needed to fetch the data from the db
    /// * `target_id` - The id of the incident to get
    pub fn get(conn: &ConnType, target_id: i32) -> Result<Self, AppError> {
        Ok(dsl_incidents.find(target_id).first(conn)?)
    }

    /// Insert a new Incidents inside the database
    /// # Params
    /// * `conn` - The r2d2 connection needed to fetch the data from the db
    /// * `incidents` - The Incidents struct containing the new incident information
    pub fn insert(conn: &ConnType, incidents: &[IncidentsDTO]) -> Result<(), AppError> {
        insert_into(dsl_incidents).values(incidents).execute(conn)?;
        Ok(())
    }

    /// Insert a new Incidents inside the database and return the inserted row
    /// # Params
    /// * `conn` - The r2d2 connection needed to fetch the data from the db
    /// * `incidents` - The Incidents struct containing the new incident information
    pub fn ginsert(conn: &ConnType, incidents: &[IncidentsDTO]) -> Result<Self, AppError> {
        Ok(insert_into(dsl_incidents)
            .values(incidents)
            .get_result(conn)?)
    }

    /// Remove an Incidents inside the database
    /// # Params
    /// * `conn` - The r2d2 connection needed to fetch the data from the db
    /// * `target_id` - The id of the incident to delete
    pub fn delete(conn: &ConnType, target_id: i32) -> Result<(), AppError> {
        let res = delete(dsl_incidents.filter(id.eq(target_id))).execute(conn)?;
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

    /// Update an Incidents inside the database
    /// # Params
    /// * `conn` - The r2d2 connection needed to fetch the data from the db
    /// * `incidents` - The HttpIncidents struct containing the updated incident information
    /// * `target_id` - The id of the incident to update
    pub fn update(
        conn: &ConnType,
        incidents: &IncidentsDTOUpdate,
        target_id: i32,
    ) -> Result<(), AppError> {
        let res = update(dsl_incidents.filter(id.eq(target_id)))
            .set(incidents)
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

    /// Update an Incidents inside the database and return the updated Struct
    /// # Params
    /// * `conn` - The r2d2 connection needed to fetch the data from the db
    /// * `incidents` - The HttpIncidents struct containing the updated incident information
    /// * `target_id` - The id of the incident to update
    pub fn gupdate(
        conn: &ConnType,
        incidents: &IncidentsDTOUpdate,
        target_id: i32,
    ) -> Result<Self, AppError> {
        Ok(update(dsl_incidents.filter(id.eq(target_id)))
            .set(incidents)
            .get_result(conn)?)
    }
}

/// Insertable struct (no id fields => which is auto generated)
#[derive(Insertable, Deserialize, Serialize, Debug)]
#[table_name = "incidents"]
pub struct IncidentsDTO {
    pub result: String,
    pub started_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
    pub resolved_at: Option<chrono::NaiveDateTime>,
    pub host_uuid: String,
    pub hostname: String,
    pub status: i32,
    pub severity: i32,
    pub alerts_id: String,
    pub alerts_name: String,
    pub alerts_table: String,
    pub alerts_lookup: String,
    pub alerts_warn: String,
    pub alerts_crit: String,
    pub alerts_info: Option<String>,
    pub alerts_where_clause: Option<String>,
}

/// Using a specific struct for the Update allow us to pass all as None expect the fields we want to update
#[derive(AsChangeset, Deserialize, Serialize, Debug, Default)]
#[table_name = "incidents"]
pub struct IncidentsDTOUpdate {
    pub result: Option<String>,
    pub started_at: Option<chrono::NaiveDateTime>,
    pub updated_at: Option<chrono::NaiveDateTime>,
    pub resolved_at: Option<chrono::NaiveDateTime>,
    pub host_uuid: Option<String>,
    pub hostname: Option<String>,
    pub status: Option<i32>,
    pub severity: Option<i32>,
    pub alerts_id: Option<String>,
    pub alerts_name: Option<String>,
    pub alerts_table: Option<String>,
    pub alerts_lookup: Option<String>,
    pub alerts_warn: Option<String>,
    pub alerts_crit: Option<String>,
    pub alerts_info: Option<String>,
    pub alerts_where_clause: Option<String>,
}

impl From<Incidents> for IncidentsDTO {
    fn from(incident: Incidents) -> IncidentsDTO {
        IncidentsDTO {
            result: incident.result,
            started_at: incident.started_at,
            updated_at: incident.updated_at,
            resolved_at: incident.resolved_at,
            host_uuid: incident.host_uuid,
            hostname: incident.hostname,
            status: incident.status,
            severity: incident.severity,
            alerts_id: incident.alerts_id,
            alerts_name: incident.alerts_name,
            alerts_table: incident.alerts_table,
            alerts_lookup: incident.alerts_lookup,
            alerts_warn: incident.alerts_warn,
            alerts_crit: incident.alerts_crit,
            alerts_info: incident.alerts_info,
            alerts_where_clause: incident.alerts_where_clause,
        }
    }
}

impl From<Incidents> for IncidentsDTOUpdate {
    fn from(incident: Incidents) -> IncidentsDTOUpdate {
        IncidentsDTOUpdate {
            result: Some(incident.result),
            started_at: Some(incident.started_at),
            updated_at: Some(incident.updated_at),
            resolved_at: incident.resolved_at,
            host_uuid: Some(incident.host_uuid),
            hostname: Some(incident.hostname),
            status: Some(incident.status),
            severity: Some(incident.severity),
            alerts_id: Some(incident.alerts_id),
            alerts_name: Some(incident.alerts_name),
            alerts_table: Some(incident.alerts_table),
            alerts_lookup: Some(incident.alerts_lookup),
            alerts_warn: Some(incident.alerts_warn),
            alerts_crit: Some(incident.alerts_crit),
            alerts_info: incident.alerts_info,
            alerts_where_clause: incident.alerts_where_clause,
        }
    }
}
