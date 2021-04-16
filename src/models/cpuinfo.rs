use crate::errors::AppError;
use crate::ConnType;

use super::schema::cpu_info::dsl::{cpu_info as dsl_cpuinfo, created_at, host_uuid};
use super::schema::*;
use super::{Host, HttpPostHost};

use diesel::*;
use serde::{Deserialize, Serialize};

// ========================
// DATABASE Specific struct
// ========================
#[derive(Identifiable, Queryable, Debug, Serialize, Deserialize, Associations)]
#[belongs_to(Host, foreign_key = "host_uuid")]
#[table_name = "cpu_info"]
pub struct CpuInfo {
    pub id: i64,
    pub cpu_freq: i64,
    pub host_uuid: String,
    pub created_at: chrono::NaiveDateTime,
}

impl CpuInfo {
    /// Return a Vector of CpuInfo
    /// # Params
    /// * `conn` - The r2d2 connection needed to fetch the data from the db
    /// * `uuid` - The host's uuid we want to get CpuInfo of
    /// * `size` - The number of elements to fetch
    /// * `page` - How many items you want to skip (page * size)
    pub fn get_data(
        conn: &ConnType,
        uuid: &str,
        size: i64,
        page: i64,
    ) -> Result<Vec<Self>, AppError> {
        Ok(dsl_cpuinfo
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
#[table_name = "cpu_info"]
pub struct NewCpuInfo<'a> {
    pub cpu_freq: i64,
    pub host_uuid: &'a str,
    pub created_at: chrono::NaiveDateTime,
}

impl<'a> From<&'a HttpPostHost> for NewCpuInfo<'a> {
    fn from(item: &'a HttpPostHost) -> NewCpuInfo<'a> {
        NewCpuInfo {
            cpu_freq: item.cpu_freq,
            host_uuid: &item.uuid,
            created_at: item.created_at,
        }
    }
}
