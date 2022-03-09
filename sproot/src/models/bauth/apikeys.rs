use crate::errors::AppError;
use crate::models::schema::apikeys;
use crate::models::schema::apikeys::dsl::{apikeys as dsl_apikeys, key};
use crate::ConnType;

use diesel::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Queryable, QueryableByName, Serialize, Deserialize)]
#[table_name = "apikeys"]
pub struct ApiKey {
    pub id: i32,
    pub key: String,
    pub host_uuid: String,
    pub berta: String,
}

impl ApiKey {
    /// Return a potential ApiKey
    /// # Params
    /// * `conn` - The r2d2 connection needed to fetch the data from the db
    /// * `hkey` - The apiKey, will be used for lookup
    pub fn get_entry(conn: &ConnType, hkey: &str) -> Result<Self, AppError> {
        Ok(dsl_apikeys.filter(key.eq(hkey)).first(conn)?)
    }
}
