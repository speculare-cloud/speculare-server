use crate::errors::AppError;
use crate::ConnType;

use super::schema::iostats;
use super::schema::iostats::dsl::{
    created_at, device_name, host_uuid, iostats as dsl_iostats, read_bytes, write_bytes,
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
#[table_name = "iostats"]
pub struct IoStats {
    pub id: i64,
    pub device_name: String,
    pub read_count: i64,
    pub read_bytes: i64,
    pub write_count: i64,
    pub write_bytes: i64,
    pub busy_time: i64,
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

    /// Return a Vector of IoStats between min_date and max_date
    /// # Params
    /// * `conn` - The r2d2 connection needed to fetch the data from the db
    /// * `uuid` - The host's uuid we want to get IoStats of
    /// * `size` - The number of elements to fetch
    /// * `min_date` - Min timestamp for the data to be fetched
    /// * `max_date` - Max timestamp for the data to be fetched
    pub fn get_data_dated(
        conn: &ConnType,
        uuid: &str,
        size: i64,
        min_date: chrono::NaiveDateTime,
        max_date: chrono::NaiveDateTime,
    ) -> Result<Vec<IoStatsDTORaw>, AppError> {
        let granularity = get_granularity(size);
        if granularity <= 1 {
            Ok(dsl_iostats
                .select((device_name, read_bytes, write_bytes, created_at))
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
                    (SELECT device_name, read_bytes, write_bytes, created_at as time 
                        FROM iostats 
                        WHERE host_uuid=$1 
                        ORDER BY created_at 
                        DESC LIMIT $2
                    ) 
                SELECT 
                    device_name, 
                    avg(read_bytes)::int8 as read_bytes, 
                    avg(write_bytes)::int8 as write_bytes, 
                    time::date + 
                        (extract(hour from time)::int)* '1h'::interval +
                        (extract(minute from time)::int/$3)* $4::interval +
                        (extract(second from time)::int/$5)* '$5s'::interval as created_at 
                    FROM s 
                    GROUP BY created_at,device_name 
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

#[derive(Queryable, QueryableByName, Serialize)]
#[table_name = "iostats"]
pub struct IoStatsDTORaw {
    pub device_name: String,
    pub read_bytes: i64,
    pub write_bytes: i64,
    pub created_at: chrono::NaiveDateTime,
}

// ================
// Insertable model
// ================
#[derive(Insertable)]
#[table_name = "iostats"]
pub struct IoStatsDTO<'a> {
    pub device_name: &'a str,
    pub read_count: i64,
    pub read_bytes: i64,
    pub write_count: i64,
    pub write_bytes: i64,
    pub busy_time: i64,
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
                read_count: iostat.read_count,
                read_bytes: iostat.read_bytes,
                write_count: iostat.write_count,
                write_bytes: iostat.write_bytes,
                busy_time: iostat.busy_time,
                host_uuid: &item.uuid,
                created_at: item.created_at,
            })
        }
        Some(list)
    }
}
