use super::schema::*;
use super::{Host, HttpPostHost};

use diesel::*;
use serde::{Deserialize, Serialize};

// ========================
// DATABASE Specific struct
// ========================
#[derive(Identifiable, Queryable, Debug, Serialize, Deserialize, Associations)]
#[belongs_to(Host, foreign_key = "host_uuid")]
#[table_name = "iostats"]
pub struct IoStats {
    pub id: i32,
    pub device_name: String,
    pub bytes_read: i64,
    pub bytes_wrtn: i64,
    pub host_uuid: String,
    pub created_at: chrono::NaiveDateTime,
}

// ================
// Insertable model
// ================
#[derive(Insertable)]
#[table_name = "iostats"]
pub struct NewIoStats<'a> {
    pub device_name: &'a str,
    pub bytes_read: i64,
    pub bytes_wrtn: i64,
    pub host_uuid: &'a str,
    pub created_at: chrono::NaiveDateTime,
}

pub type NewIostatsList<'a> = Vec<NewIoStats<'a>>;
impl<'a> From<&'a HttpPostHost> for Option<NewIostatsList<'a>> {
    fn from(item: &'a HttpPostHost) -> Option<NewIostatsList<'a>> {
        let iostats = item.iostats.as_ref()?;
        let mut list = Vec::with_capacity(iostats.len());
        for iostat in iostats {
            list.push(NewIoStats {
                device_name: &iostat.device_name,
                bytes_read: iostat.bytes_read,
                bytes_wrtn: iostat.bytes_wrtn,
                host_uuid: &item.uuid,
                created_at: item.created_at,
            })
        }
        Some(list)
    }
}
