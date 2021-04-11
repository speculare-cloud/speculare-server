use super::schema::*;
use super::{Host, HttpPostHost};

use diesel::*;
use serde::{Deserialize, Serialize};

// ========================
// DATABASE Specific struct
// ========================
#[derive(Identifiable, Queryable, Debug, Serialize, Deserialize, Associations)]
#[belongs_to(Host, foreign_key = "host_uuid")]
#[table_name = "disks"]
pub struct Disks {
    pub id: i64,
    pub disk_name: String,
    pub mount_point: String,
    pub total_space: i64,
    pub avail_space: i64,
    pub host_uuid: String,
    pub created_at: chrono::NaiveDateTime,
}

// ================
// Insertable model
// ================
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

pub type NewDisksList<'a> = Vec<NewDisks<'a>>;
impl<'a> From<&'a HttpPostHost> for Option<NewDisksList<'a>> {
    fn from(item: &'a HttpPostHost) -> Option<NewDisksList<'a>> {
        let disks = item.disks.as_ref()?;
        let mut list = Vec::with_capacity(disks.len());
        for disk in disks {
            list.push(NewDisks {
                disk_name: &disk.name,
                mount_point: &disk.mount_point,
                total_space: disk.total_space,
                avail_space: disk.avail_space,
                host_uuid: &item.uuid,
                created_at: item.created_at,
            })
        }
        Some(list)
    }
}
