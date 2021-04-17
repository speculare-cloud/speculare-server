use crate::errors::AppError;
use crate::ConnType;

use super::schema::memory::dsl::{created_at, host_uuid, memory as dsl_memory};
use super::schema::*;
use super::{Host, HttpPostHost};

use diesel::*;
use serde::{Deserialize, Serialize};

// ========================
// DATABASE Specific struct
// ========================
#[derive(Identifiable, Queryable, Debug, Serialize, Deserialize, Associations)]
#[belongs_to(Host, foreign_key = "host_uuid")]
#[table_name = "memory"]
pub struct Memory {
    pub id: i64,
    pub total_virt: i64,
    pub avail_virt: i64,
    pub total_swap: i64,
    pub avail_swap: i64,
    pub host_uuid: String,
    pub created_at: chrono::NaiveDateTime,
}

impl Memory {
    /// Return a Vector of Memory
    /// # Params
    /// * `conn` - The r2d2 connection needed to fetch the data from the db
    /// * `uuid` - The host's uuid we want to get Memory of
    /// * `size` - The number of elements to fetch
    /// * `page` - How many items you want to skip (page * size)
    pub fn get_data(
        conn: &ConnType,
        uuid: &str,
        size: i64,
        page: i64,
    ) -> Result<Vec<Self>, AppError> {
        Ok(dsl_memory
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
#[table_name = "memory"]
pub struct NewMemory<'a> {
    pub total_virt: i64,
    pub avail_virt: i64,
    pub total_swap: i64,
    pub avail_swap: i64,
    pub host_uuid: &'a str,
    pub created_at: chrono::NaiveDateTime,
}

impl<'a> From<&'a HttpPostHost> for Option<NewMemory<'a>> {
    fn from(item: &'a HttpPostHost) -> Option<NewMemory<'a>> {
        let memory = item.memory.as_ref()?;
        Some(NewMemory {
            total_virt: memory.total_virt as i64,
            avail_virt: memory.avail_virt as i64,
            total_swap: memory.total_swap as i64,
            avail_swap: memory.avail_swap as i64,
            host_uuid: &item.uuid,
            created_at: item.created_at,
        })
    }
}
