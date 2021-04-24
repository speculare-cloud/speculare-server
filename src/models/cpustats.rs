use crate::errors::AppError;
use crate::ConnType;

use super::schema::cpustats;
use super::schema::cpustats::dsl::{cpustats as dsl_cpustats, created_at, host_uuid};
use super::{Host, HttpPostHost};

use diesel::*;
use serde::{Deserialize, Serialize};

// ========================
// DATABASE Specific struct
// ========================
#[derive(Identifiable, Queryable, Debug, Serialize, Deserialize, Associations)]
#[belongs_to(Host, foreign_key = "host_uuid")]
#[table_name = "cpustats"]
pub struct CpuStats {
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

impl CpuStats {
    /// Return a Vector of CpuStats
    /// # Params
    /// * `conn` - The r2d2 connection needed to fetch the data from the db
    /// * `uuid` - The host's uuid we want to get CpuStats of
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
            Ok(dsl_cpustats
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
            Ok(dsl_cpustats
                .filter(host_uuid.eq(uuid))
                .limit(size)
                .offset(page * size)
                .order_by(created_at.desc())
                .load(conn)?)
        }
    }
}

// ================
// Insertable model
// ================
#[derive(Insertable)]
#[table_name = "cpustats"]
pub struct CpuStatsDTO<'a> {
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

impl<'a> From<&'a HttpPostHost> for Option<CpuStatsDTO<'a>> {
    fn from(item: &'a HttpPostHost) -> Option<CpuStatsDTO<'a>> {
        let cpustats = item.cpu_stats.as_ref()?;
        Some(CpuStatsDTO {
            cuser: cpustats.user,
            nice: cpustats.nice,
            system: cpustats.system,
            idle: cpustats.idle,
            iowait: cpustats.iowait,
            irq: cpustats.irq,
            softirq: cpustats.softirq,
            steal: cpustats.steal,
            guest: cpustats.guest,
            guest_nice: cpustats.guest_nice,
            host_uuid: &item.uuid,
            created_at: item.created_at,
        })
    }
}
