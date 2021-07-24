use crate::errors::AppError;
use crate::ConnType;

use crate::models::schema::cpustats;
use crate::models::schema::cpustats::dsl::{
    cpustats as dsl_cpustats, created_at, ctx_switches, host_uuid, interrupts, processes,
    procs_blocked, procs_running, soft_interrupts,
};
use crate::models::{get_granularity, get_query_range_values, HttpPostHost};

use diesel::{
    pg::expression::extensions::IntervalDsl,
    sql_types::{Int8, Interval, Text},
    *,
};
use serde::{Deserialize, Serialize};

// ========================
// DATABASE Specific struct
// ========================
#[derive(Identifiable, Queryable, Debug, Serialize, Deserialize)]
#[table_name = "cpustats"]
pub struct CpuStats {
    pub id: i64,
    pub interrupts: i64,
    pub ctx_switches: i64,
    pub soft_interrupts: i64,
    pub processes: i64,
    pub procs_running: i64,
    pub procs_blocked: i64,
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
                .select((
                    interrupts,
                    ctx_switches,
                    soft_interrupts,
                    processes,
                    procs_running,
                    procs_blocked,
                    created_at,
                ))
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

            // Dummy require to ensure no issue if table name change.
            // If the table's name is to be changed, we have to change it from the sql_query below.
            {
                #[allow(unused_imports)]
                use crate::models::schema::cpustats;
            }

            // TODO - Should we use MAX instead of AVG for proc* ?
            Ok(sql_query(
                "
                WITH s AS 
                    (SELECT interrupts, ctx_switches, soft_interrupts, processes, procs_running, procs_blocked, created_at as time 
                        FROM cpustats 
                        WHERE host_uuid=$1 
                        ORDER BY created_at 
                        DESC LIMIT $2
                    )
                SELECT 
                    avg(interrupts)::int8 as interrupts, 
                    avg(ctx_switches)::int8 as ctx_switches, 
                    avg(soft_interrupts)::int8 as soft_interrupts,
                    avg(processes)::int8 as processes,
                    avg(procs_running)::int8 as procs_running,
                    avg(procs_blocked)::int8 as procs_blocked, 
                    time::date + 
                        (extract(hour from time)::int)* '1h'::interval +
                        (extract(minute from time)::int/$3)* $4 +
                        (extract(second from time)::int/$5)* $6 as created_at 
                    FROM s 
                    GROUP BY created_at 
                    ORDER BY created_at DESC",
            )
            .bind::<Text, _>(uuid)
            .bind::<Int8, _>(size)
            .bind::<Int8, _>(min)
            .bind::<Interval, _>(min.minute() + sec_supp.second())
            .bind::<Int8, _>(granularity)
            .bind::<Interval, _>(granularity.second())
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
    pub processes: i64,
    pub procs_running: i64,
    pub procs_blocked: i64,
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
    pub processes: i64,
    pub procs_running: i64,
    pub procs_blocked: i64,
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
            processes: cpustats.processes,
            procs_running: cpustats.procs_running,
            procs_blocked: cpustats.procs_blocked,
            host_uuid: &item.uuid,
            created_at: item.created_at,
        })
    }
}
