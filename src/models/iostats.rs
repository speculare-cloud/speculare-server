use crate::errors::AppError;
use crate::ConnType;

use super::schema::iostats;
use super::schema::iostats::dsl::{created_at, device_name, host_uuid, iostats as dsl_iostats};
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
        min_date: Option<chrono::NaiveDateTime>,
        max_date: Option<chrono::NaiveDateTime>,
    ) -> Result<Vec<Self>, AppError> {
        if min_date.is_some() && max_date.is_some() {
            Ok(dsl_iostats
                .filter(
                    host_uuid.eq(uuid).and(
                        created_at
                            .gt(min_date.unwrap())
                            .and(created_at.le(max_date.unwrap())),
                    ),
                )
                .limit(size)
                .offset(page * size)
                .order_by(created_at.desc())
                .load(conn)?)
        } else {
            Ok(dsl_iostats
                .filter(host_uuid.eq(uuid))
                .limit(size)
                .offset(page * size)
                .order_by(created_at.desc())
                .load(conn)?)
        }
    }

    /// Return the numbers of iostats the host have
    /// # Params
    /// * `conn` - The r2d2 connection needed to fetch the data from the db
    /// * `uuid` - The host's uuid we want to get the number of iostats of
    /// * `size` - The number of elements to fetch
    pub fn count(conn: &ConnType, uuid: &str, size: i64) -> Result<usize, AppError> {
        let mut devices = dsl_iostats
            .select(device_name)
            .filter(host_uuid.eq(uuid))
            .limit(size)
            .order_by(created_at.desc())
            .load::<String>(conn)?;
        devices.sort();
        devices.dedup();
        Ok(devices.len())
    }
}

// ================
// Insertable model
// ================
#[derive(Insertable)]
#[table_name = "iostats"]
pub struct IoStatsDTO<'a> {
    pub device_name: &'a str,
    pub bytes_read: i64,
    pub bytes_wrtn: i64,
    pub host_uuid: &'a str,
    pub created_at: chrono::NaiveDateTime,
}

pub type IostatsDTOList<'a> = Vec<IoStatsDTO<'a>>;
impl<'a> From<&'a HttpPostHost> for Option<IostatsDTOList<'a>> {
    fn from(item: &'a HttpPostHost) -> Option<IostatsDTOList<'a>> {
        let iostats = item.iostats.as_ref()?;
        let mut list = Vec::with_capacity(iostats.len());
        for iostat in iostats {
            list.push(IoStatsDTO {
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
