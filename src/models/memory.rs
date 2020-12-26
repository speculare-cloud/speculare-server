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
    pub id: i32,
    pub total_virt: i64,
    pub avail_virt: i64,
    pub total_swap: i64,
    pub avail_swap: i64,
    pub host_uuid: String,
    pub created_at: chrono::NaiveDateTime,
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
