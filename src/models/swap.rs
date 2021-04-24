use crate::errors::AppError;
use crate::ConnType;

use super::schema::swap;
use super::schema::swap::dsl::{created_at, host_uuid, swap as dsl_swap};
use super::{Host, HttpPostHost};

use diesel::*;
use serde::{Deserialize, Serialize};

// ========================
// DATABASE Specific struct
// ========================
#[derive(Identifiable, Queryable, Debug, Serialize, Deserialize, Associations)]
#[belongs_to(Host, foreign_key = "host_uuid")]
#[table_name = "swap"]
pub struct Swap {
    pub id: i64,
    pub total: i64,
    pub free: i64,
    pub used: i64,
    pub host_uuid: String,
    pub created_at: chrono::NaiveDateTime,
}

impl Swap {
    /// Return a Vector of Swap
    /// # Params
    /// * `conn` - The r2d2 connection needed to fetch the data from the db
    /// * `uuid` - The host's uuid we want to get Swap of
    /// * `size` - The number of elements to fetch
    /// * `page` - How many items you want to skip (page * size)
    pub fn get_data(
        conn: &ConnType,
        uuid: &str,
        size: i64,
        page: i64,
    ) -> Result<Vec<Self>, AppError> {
        Ok(dsl_swap
            .filter(host_uuid.eq(uuid))
            .limit(size)
            .offset(page * size)
            .order_by(created_at.desc())
            .load(conn)?)
    }
}

// ================
// Insertable model
// ================
#[derive(Insertable)]
#[table_name = "swap"]
pub struct SwapDTO<'a> {
    pub total: i64,
    pub free: i64,
    pub used: i64,
    pub host_uuid: &'a str,
    pub created_at: chrono::NaiveDateTime,
}

impl<'a> From<&'a HttpPostHost> for Option<SwapDTO<'a>> {
    fn from(item: &'a HttpPostHost) -> Option<SwapDTO<'a>> {
        let swap = item.swap.as_ref()?;
        Some(SwapDTO {
            total: swap.total as i64,
            free: swap.free as i64,
            used: swap.used as i64,
            host_uuid: &item.uuid,
            created_at: item.created_at,
        })
    }
}
