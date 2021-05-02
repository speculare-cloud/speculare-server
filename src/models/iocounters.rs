use crate::errors::AppError;
use crate::ConnType;

use super::schema::iocounters;
use super::schema::iocounters::dsl::{
    created_at, host_uuid, interface, iocounters as dsl_iocounters,
};
use super::{Host, HttpPostHost};

use diesel::*;
use serde::{Deserialize, Serialize};

// ========================
// DATABASE Specific struct
// ========================
#[derive(Identifiable, Queryable, Debug, Serialize, Deserialize, Associations)]
#[belongs_to(Host, foreign_key = "host_uuid")]
#[table_name = "iocounters"]
pub struct IoCounters {
    pub id: i64,
    pub interface: String,
    pub rx_bytes: i64,
    pub rx_packets: i64,
    pub rx_errs: i64,
    pub rx_drop: i64,
    pub tx_bytes: i64,
    pub tx_packets: i64,
    pub tx_errs: i64,
    pub tx_drop: i64,
    pub host_uuid: String,
    pub created_at: chrono::NaiveDateTime,
}

impl IoCounters {
    /// Return a Vector of IoCounters
    /// # Params
    /// * `conn` - The r2d2 connection needed to fetch the data from the db
    /// * `uuid` - The host's uuid we want to get IoCounters of
    /// * `size` - The number of elements to fetch
    /// * `page` - How many items you want to skip (page * size)
    pub fn get_data(
        conn: &ConnType,
        uuid: &str,
        size: i64,
        page: i64,
    ) -> Result<Vec<Self>, AppError> {
        Ok(dsl_iocounters
            .filter(host_uuid.eq(uuid))
            .limit(size)
            .offset(page * size)
            .order_by(created_at.desc())
            .load(conn)?)
    }

    /// Return a Vector of IoCounters between min_date and max_date
    /// # Params
    /// * `conn` - The r2d2 connection needed to fetch the data from the db
    /// * `uuid` - The host's uuid we want to get IoCounters of
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
        Ok(dsl_iocounters
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

    /// Return the numbers of iostats the host have
    /// # Params
    /// * `conn` - The r2d2 connection needed to fetch the data from the db
    /// * `uuid` - The host's uuid we want to get the number of iostats of
    /// * `size` - The number of elements to fetch
    pub fn count(conn: &ConnType, uuid: &str, size: i64) -> Result<usize, AppError> {
        let mut devices = dsl_iocounters
            .select(interface)
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
#[table_name = "iocounters"]
pub struct IoCountersDTO<'a> {
    pub interface: &'a str,
    pub rx_bytes: i64,
    pub rx_packets: i64,
    pub rx_errs: i64,
    pub rx_drop: i64,
    pub tx_bytes: i64,
    pub tx_packets: i64,
    pub tx_errs: i64,
    pub tx_drop: i64,
    pub host_uuid: &'a str,
    pub created_at: chrono::NaiveDateTime,
}

pub type IoCountersDTOList<'a> = Vec<IoCountersDTO<'a>>;
impl<'a> From<&'a HttpPostHost> for Option<IoCountersDTOList<'a>> {
    fn from(item: &'a HttpPostHost) -> Option<IoCountersDTOList<'a>> {
        let iocounters = item.iocounters.as_ref()?;
        let mut list = Vec::with_capacity(iocounters.len());
        for iocounter in iocounters {
            list.push(IoCountersDTO {
                interface: &iocounter.interface,
                rx_bytes: iocounter.rx_bytes,
                rx_packets: iocounter.rx_packets,
                rx_errs: iocounter.rx_errs,
                rx_drop: iocounter.rx_drop,
                tx_bytes: iocounter.tx_bytes,
                tx_packets: iocounter.tx_packets,
                tx_errs: iocounter.tx_errs,
                tx_drop: iocounter.tx_drop,
                host_uuid: &item.uuid,
                created_at: item.created_at,
            })
        }
        Some(list)
    }
}
