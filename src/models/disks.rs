use super::schema::*;
use super::Host;

use diesel::*;
use serde::{Deserialize, Serialize};

// DATABASE Specific struct
#[derive(Identifiable, Queryable, Debug, Serialize, Deserialize, Associations)]
#[belongs_to(Host, foreign_key = "host_uuid")]
#[table_name = "disks"]
pub struct Disks {
    pub id: i32,
    pub disk_name: String,
    pub mount_point: String,
    pub total_space: i64,
    pub avail_space: i64,
    pub host_uuid: String,
    pub created_at: chrono::NaiveDateTime,
}

// Insertable models
#[derive(Insertable)]
#[table_name = "disks"]
pub struct NewDisks<'a> {
    pub disk_name: &'a str,
    pub mount_point: &'a str,
    pub total_space: i64,
    pub avail_space: i64,
    pub host_uuid: &'a str,
    pub created_at: chrono::NaiveDateTime,
}
