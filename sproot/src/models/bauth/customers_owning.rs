use crate::errors::AppError;
use crate::models::schema::customers_owning;
use crate::models::schema::customers_owning::dsl::{
    customer_id, customers_owning as dsl_customers_owning, host_uuid,
};
use crate::ConnType;

use diesel::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Queryable, QueryableByName, Serialize, Deserialize)]
#[table_name = "customers_owning"]
pub struct CustomersOwning {
    pub id: i32,
    pub customer_id: String,
    pub host_uuid: String,
}

impl CustomersOwning {
    /// Check if the entry exists for that pair of customer ID and host_uuid
    /// # Params
    /// * `conn` - The r2d2 connection needed to fetch the data from the db
    /// * `cid` - The customer ID
    /// * `uuid` - The host_uuid
    pub fn entry_exists(conn: &ConnType, cid: &str, uuid: &str) -> Result<bool, AppError> {
        let res: Option<Self> = dsl_customers_owning
            .filter(customer_id.eq(cid).and(host_uuid.eq(uuid)))
            .first(conn)
            .optional()?;

        Ok(res.is_some())
    }
}
