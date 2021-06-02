use crate::errors::AppError;
use crate::ConnType;

use super::schema::cpustats;
use super::schema::cpustats::dsl::{
    cpustats as dsl_cpustats, created_at, ctx_switches, host_uuid, interrupts, soft_interrupts,
};
use super::{get_granularity, get_query_range_values, Host, HttpPostHost};

use diesel::{
    sql_types::{Int8, Text},
    *,
};
use serde::{Deserialize, Serialize};

// ========================
// DATABASE Specific struct
// ========================
#[derive(Identifiable, Queryable, Debug, Serialize, Deserialize, Associations)]
#[belongs_to(Host, foreign_key = "host_uuid")]
#[table_name = "cpustats"]
pub struct CpuStats {
    pub id: i64,
    pub interrupts: i64,
    pub ctx_switches: i64,
    pub soft_interrupts: i64,
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
    ) -> Result<Vec<Self>, AppError> {
        Ok(dsl_cpustats
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
    /// * `min_date` - Min timestamp for the data to be fetched
    /// * `max_date` - Max timestamp for the data to be fetched
    pub fn get_data_dated(
        conn: &ConnType,
        uuid: &str,
        size: i64,
        min_date: chrono::NaiveDateTime,
        max_date: chrono::NaiveDateTime,
    ) -> Result<Vec<CpuStatsDTORaw>, AppError> {
        let granularity = get_granularity(size);
        if granularity <= 1 {
            Ok(dsl_cpustats
                .select((interrupts, ctx_switches, soft_interrupts, created_at))
                .filter(
                    host_uuid
                        .eq(uuid)
                        .and(created_at.gt(min_date).and(created_at.le(max_date))),
                )
                .limit(size)
                .order_by(created_at.desc())
                .load(conn)?)
        } else {
            // TODO - Add min_date & max_date in the QUERY
            // Compute values if granularity > 60
            let (min, sec_supp, granularity) = get_query_range_values(granularity);
            // Prepare and run the query
            Ok(sql_query(
                "
                WITH s AS 
                    (SELECT interrupts, ctx_switches, soft_interrupts, created_at as time 
                        FROM cpustats 
                        WHERE host_uuid=$1 
                        ORDER BY created_at 
                        DESC LIMIT $2
                    )
                SELECT 
                    avg(interrupts)::int8 as interrupts, 
                    avg(ctx_switches)::int8 as ctx_switches, 
                    avg(soft_interrupts)::int8 as soft_interrupts, 
                    time::date + 
                        (extract(hour from time)::int)* '1h'::interval +
                        (extract(minute from time)::int/$3)* $4::interval +
                        (extract(second from time)::int/$5)* '$5s'::interval as created_at 
                    FROM s 
                    GROUP BY created_at 
                    ORDER BY created_at DESC",
            )
            .bind::<Text, _>(uuid)
            .bind::<Int8, _>(size)
            .bind::<Int8, _>(min)
            .bind::<Text, _>(format!("{}m{}s", min, sec_supp))
            .bind::<Int8, _>(granularity)
            .load(conn)?)
        }
    }
}

#[derive(Queryable, QueryableByName, Serialize)]
#[table_name = "cpustats"]
pub struct CpuStatsDTORaw {
    pub interrupts: i64,
    pub ctx_switches: i64,
    pub soft_interrupts: i64,
    pub created_at: chrono::NaiveDateTime,
}

// ================
// Insertable model
// ================
#[derive(Insertable)]
#[table_name = "cpustats"]
pub struct CpuStatsDTO<'a> {
    pub interrupts: i64,
    pub ctx_switches: i64,
    pub soft_interrupts: i64,
    pub host_uuid: &'a str,
    pub created_at: chrono::NaiveDateTime,
}

impl<'a> From<&'a HttpPostHost> for Option<CpuStatsDTO<'a>> {
    fn from(item: &'a HttpPostHost) -> Option<CpuStatsDTO<'a>> {
        let cpustats = item.cpu_stats.as_ref()?;
        Some(CpuStatsDTO {
            interrupts: cpustats.interrupts,
            ctx_switches: cpustats.ctx_switches,
            soft_interrupts: cpustats.soft_interrupts,
            host_uuid: &item.uuid,
            created_at: item.created_at,
        })
    }
}