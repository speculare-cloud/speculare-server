use crate::errors::AppError;
use crate::ConnType;

use super::schema::memory;
use super::schema::memory::dsl::{
    buffers, cached, created_at, free, host_uuid, memory as dsl_memory, used,
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
#[table_name = "memory"]
pub struct Memory {
    pub id: i64,
    pub total: i64,
    pub free: i64,
    pub used: i64,
    pub shared: i64,
    pub buffers: i64,
    pub cached: i64,
    pub host_uuid: String,
    pub created_at: chrono::NaiveDateTime,
}

impl Memory {
    /// Return a Vector of Memory
    /// # Params
    /// * `conn` - The r2d2 connection needed to fetch the data from the db
    /// * `uuid` - The host's uuid we want to get Memory of
    /// * `size` - The number of elements to fetch
    /// * `page` - How many items you want to skip (page * size)
    pub fn get_data(
        conn: &ConnType,
        uuid: &str,
        size: i64,
        page: i64,
    ) -> Result<Vec<Self>, AppError> {
        Ok(dsl_memory
            .filter(host_uuid.eq(uuid))
            .limit(size)
            .offset(page * size)
            .order_by(created_at.desc())
            .load(conn)?)
    }

    /// Return a Vector of Memory between min_date and max_date
    /// # Params
    /// * `conn` - The r2d2 connection needed to fetch the data from the db
    /// * `uuid` - The host's uuid we want to get Memory of
    /// * `size` - The number of elements to fetch
    /// * `min_date` - Min timestamp for the data to be fetched
    /// * `max_date` - Max timestamp for the data to be fetched
    pub fn get_data_dated(
        conn: &ConnType,
        uuid: &str,
        size: i64,
        min_date: chrono::NaiveDateTime,
        max_date: chrono::NaiveDateTime,
    ) -> Result<Vec<MemoryDTORaw>, AppError> {
        let granularity = get_granularity(size);
        if granularity <= 1 {
            Ok(dsl_memory
                .select((free, used, buffers, cached, created_at))
                .filter(
                    host_uuid
                        .eq(uuid)
                        .and(created_at.gt(min_date).and(created_at.le(max_date))),
                )
                .limit(size)
                .order_by(created_at.desc())
                .load(conn)?)
        } else {
            // Compute values if granularity > 60
            let (min, sec_supp, granularity) = get_query_range_values(granularity);
            // Prepare and run the query
            Ok(sql_query(
                "
                WITH s AS 
                    (SELECT free, used, buffers, cached, created_at as time 
                        FROM memory 
                        WHERE host_uuid=$1 
                        ORDER BY created_at 
                        DESC LIMIT $2
                    ) 
                SELECT 
                    avg(free)::int8 as free, 
                    avg(used)::int8 as used, 
                    avg(buffers)::int8 as buffers, 
                    avg(cached)::int8 as cached, 
                    time::date + 
                        (extract(hour from time)::int)* '1h'::interval +
                        (extract(minute from time)::int/3$)* '3$m4$s'::interval +
                        (extract(second from time)::int/5$)* '5$s'::interval as created_at 
                    FROM s 
                    GROUP BY created_at 
                    ORDER BY created_at DESC",
            )
            .bind::<Text, _>(uuid)
            .bind::<Int8, _>(size)
            .bind::<Int8, _>(min)
            .bind::<Int8, _>(sec_supp)
            .bind::<Int8, _>(granularity)
            .load(conn)?)
        }
    }
}

#[derive(Queryable, QueryableByName, Serialize)]
#[table_name = "memory"]
pub struct MemoryDTORaw {
    pub free: i64,
    pub used: i64,
    pub buffers: i64,
    pub cached: i64,
    pub created_at: chrono::NaiveDateTime,
}

// ================
// Insertable model
// ================
#[derive(Insertable)]
#[table_name = "memory"]
pub struct MemoryDTO<'a> {
    pub total: i64,
    pub free: i64,
    pub used: i64,
    pub shared: i64,
    pub buffers: i64,
    pub cached: i64,
    pub host_uuid: &'a str,
    pub created_at: chrono::NaiveDateTime,
}

impl<'a> From<&'a HttpPostHost> for Option<MemoryDTO<'a>> {
    fn from(item: &'a HttpPostHost) -> Option<MemoryDTO<'a>> {
        let memory = item.memory.as_ref()?;
        Some(MemoryDTO {
            total: memory.total as i64,
            free: memory.free as i64,
            used: memory.used as i64,
            shared: memory.shared as i64,
            buffers: memory.buffers as i64,
            cached: memory.cached as i64,
            host_uuid: &item.uuid,
            created_at: item.created_at,
        })
    }
}
