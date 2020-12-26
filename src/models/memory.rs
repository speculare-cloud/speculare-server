use super::schema::*;
use super::Host;

use diesel::*;
use serde::{Deserialize, Serialize};

// DATABASE Specific struct
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

// Insertable models
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
