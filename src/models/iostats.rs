use crate::errors::AppError;
use crate::ConnType;

use super::schema::iostats::dsl::{created_at, device_name, host_uuid, iostats as dsl_iostats};
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
    pub id: i64,
    pub device_name: String,
    pub bytes_read: i64,
    pub bytes_wrtn: i64,
    pub host_uuid: String,
    pub created_at: chrono::NaiveDateTime,
}

impl IoStats {
    /// Return a Vector of IoStats
    /// # Params
    /// * `conn` - The r2d2 connection needed to fetch the data from the db
    /// * `uuid` - The host's uuid we want to get IoStats of
    /// * `size` - The number of elements to fetch
    /// * `page` - How many items you want to skip (page * size)
    pub fn get_data(
        conn: &ConnType,
        uuid: &str,
        size: i64,
        page: i64,
    ) -> Result<Vec<Self>, AppError> {
        Ok(dsl_iostats
            .filter(host_uuid.eq(uuid))
            .limit(size)
            .offset(page * size)
            .order_by(created_at.desc())
            .load(conn)?)
    }

    /// Return the numbers of iostats the host have
    /// # Params
    /// * `conn` - The r2d2 connection needed to fetch the data from the db
    /// * `uuid` - The host's uuid we want to get the number of iostats of
    /// * `size` - The number of elements to fetch
    pub fn count(conn: &ConnType, uuid: &str, size: i64) -> Result<i64, AppError> {
        Ok(dsl_iostats
            .filter(host_uuid.eq(uuid))
            .limit(size)
            .order_by(created_at.desc())
            .then_order_by(device_name.desc())
            .distinct_on(device_name)
            .count()
            .get_result::<i64>(conn)?)
    }
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
