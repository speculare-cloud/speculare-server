use crate::errors::AppError;
use crate::ConnType;

use super::schema::memory;
use super::schema::memory::dsl::{created_at, host_uuid, memory as dsl_memory};
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
    pub total: i64,
    pub free: i64,
    pub used: i64,
    pub shared: i64,
    pub buffers: i64,
    pub cached: i64,
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

    /// Return a Vector of Memory between min_date and max_date
    /// # Params
    /// * `conn` - The r2d2 connection needed to fetch the data from the db
    /// * `uuid` - The host's uuid we want to get Memory of
    /// * `size` - The number of elements to fetch
    /// * `page` - How many items you want to skip (page * size)
    /// * `min_date` - Min timestamp for the data to be fetched
    /// * `max_date` - Max timestamp for the data to be fetched
    pub fn get_data_dated(
        conn: &ConnType,
        uuid: &str,
        size: i64,
        page: i64,
        min_date: chrono::NaiveDateTime,
        max_date: chrono::NaiveDateTime,
    ) -> Result<Vec<Self>, AppError> {
        Ok(dsl_memory
            .filter(
                host_uuid
                    .eq(uuid)
                    .and(created_at.gt(min_date).and(created_at.le(max_date))),
            )
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
pub struct MemoryDTO<'a> {
    pub total: i64,
    pub free: i64,
    pub used: i64,
    pub shared: i64,
    pub buffers: i64,
    pub cached: i64,
    pub host_uuid: &'a str,
    pub created_at: chrono::NaiveDateTime,
}

impl<'a> From<&'a HttpPostHost> for Option<MemoryDTO<'a>> {
    fn from(item: &'a HttpPostHost) -> Option<MemoryDTO<'a>> {
        let memory = item.memory.as_ref()?;
        Some(MemoryDTO {
            total: memory.total as i64,
            free: memory.free as i64,
            used: memory.used as i64,
            shared: memory.shared as i64,
            buffers: memory.buffers as i64,
            cached: memory.cached as i64,
            host_uuid: &item.uuid,
            created_at: item.created_at,
        })
    }
}
