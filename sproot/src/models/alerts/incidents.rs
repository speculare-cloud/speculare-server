use crate::errors::AppError;
use crate::ConnType;

use crate::models::schema::incidents;
use crate::models::schema::incidents::dsl::{
    host_uuid, id, incidents as dsl_incidents, updated_at,
};

use super::HttpIncidents;

use diesel::*;
use serde::{Deserialize, Serialize};

/// Struct to hold information about incidents
/// Yes it's a lot of duplicate from the Alerts but as the Alerts can be updated
/// we need to store a snapshot of the configuration of the said alerts at the
/// time the incidents was created.
#[derive(Identifiable, Queryable, Debug, Serialize, Deserialize, Clone, Insertable)]
#[table_name = "incidents"]
pub struct Incidents {
    pub id: i32,
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
    pub fn insert(conn: &ConnType, incidents: &[Incidents]) -> Result<(), AppError> {
        insert_into(dsl_incidents).values(incidents).execute(conn)?;
        Ok(())
    }

    /// Remove an Incidents inside the database
    /// # Params
    /// * `conn` - The r2d2 connection needed to fetch the data from the db
    /// * `target_id` - The id of the incident to delete
    pub fn delete(conn: &ConnType, target_id: i32) -> Result<(), AppError> {
        delete(dsl_incidents.filter(id.eq(target_id))).execute(conn)?;
        Ok(())
    }

    /// Update an Incidents inside the database
    /// # Params
    /// * `conn` - The r2d2 connection needed to fetch the data from the db
    /// * `incidents` - The HttpIncidents struct containing the updated incident information
    /// * `target_id` - The id of the incident to update
    pub fn update(
        conn: &ConnType,
        incidents: &HttpIncidents,
        target_id: i32,
    ) -> Result<(), AppError> {
        update(dsl_incidents.filter(id.eq(target_id)))
            .set(incidents)
            .execute(conn)?;
        Ok(())
    }
}
