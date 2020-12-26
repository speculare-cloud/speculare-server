use super::schema::*;
use super::{Host, HttpPostHost};

use diesel::*;
use serde::{Deserialize, Serialize};

// DATABASE Specific struct
#[derive(Identifiable, Queryable, Debug, Serialize, Deserialize, Associations)]
#[belongs_to(Host, foreign_key = "host_uuid")]
#[table_name = "cpu_info"]
pub struct CpuInfo {
    pub id: i32,
    pub cpu_freq: i64,
    pub host_uuid: String,
    pub created_at: chrono::NaiveDateTime,
}

// Insertable models
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
