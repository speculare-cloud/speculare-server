use crate::errors::AppError;
use crate::ConnType;

use super::schema::cputimes;
use super::schema::cputimes::dsl::{cputimes as dsl_cputimes, created_at, host_uuid};
use super::{Host, HttpPostHost};

use diesel::*;
use serde::{Deserialize, Serialize};

// ========================
// DATABASE Specific struct
// ========================
#[derive(Identifiable, Queryable, Debug, Serialize, Deserialize, Associations)]
#[belongs_to(Host, foreign_key = "host_uuid")]
#[table_name = "cputimes"]
pub struct CpuTimes {
    pub id: i64,
    pub cuser: i64,
    pub nice: i64,
    pub system: i64,
    pub idle: i64,
    pub iowait: i64,
    pub irq: i64,
    pub softirq: i64,
    pub steal: i64,
    pub guest: i64,
    pub guest_nice: i64,
    pub host_uuid: String,
    pub created_at: chrono::NaiveDateTime,
}

impl CpuTimes {
    /// Return a Vector of CpuTimes
    /// # Params
    /// * `conn` - The r2d2 connection needed to fetch the data from the db
    /// * `uuid` - The host's uuid we want to get CpuTimes of
    /// * `size` - The number of elements to fetch
    /// * `page` - How many items you want to skip (page * size)
    pub fn get_data(
        conn: &ConnType,
        uuid: &str,
        size: i64,
        page: i64,
    ) -> Result<Vec<Self>, AppError> {
        Ok(dsl_cputimes
            .filter(host_uuid.eq(uuid))
            .limit(size)
            .offset(page * size)
            .order_by(created_at.desc())
            .load(conn)?)
    }

    /// Return a Vector of CpuTimes between min_date and max_date
    /// # Params
    /// * `conn` - The r2d2 connection needed to fetch the data from the db
    /// * `uuid` - The host's uuid we want to get CpuTimes of
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
        Ok(dsl_cputimes
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
}

// ================
// Insertable model
// ================
#[derive(Insertable)]
#[table_name = "cputimes"]
pub struct CpuTimesDTO<'a> {
    pub cuser: i64,
    pub nice: i64,
    pub system: i64,
    pub idle: i64,
    pub iowait: i64,
    pub irq: i64,
    pub softirq: i64,
    pub steal: i64,
    pub guest: i64,
    pub guest_nice: i64,
    pub host_uuid: &'a str,
    pub created_at: chrono::NaiveDateTime,
}

impl<'a> From<&'a HttpPostHost> for Option<CpuTimesDTO<'a>> {
    fn from(item: &'a HttpPostHost) -> Option<CpuTimesDTO<'a>> {
        let cputimes = item.cpu_times.as_ref()?;
        Some(CpuTimesDTO {
            cuser: cputimes.user,
            nice: cputimes.nice,
            system: cputimes.system,
            idle: cputimes.idle,
            iowait: cputimes.iowait,
            irq: cputimes.irq,
            softirq: cputimes.softirq,
            steal: cputimes.steal,
            guest: cputimes.guest,
            guest_nice: cputimes.guest_nice,
            host_uuid: &item.uuid,
            created_at: item.created_at,
        })
    }
}
