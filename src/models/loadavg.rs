use crate::errors::AppError;
use crate::ConnType;

use super::schema::loadavg;
use super::schema::loadavg::dsl::{
    created_at, fifteen, five, host_uuid, loadavg as dsl_loadavg, one,
};
use super::{get_granularity, Host, HttpPostHost};

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
#[table_name = "loadavg"]
pub struct LoadAvg {
    pub id: i64,
    pub one: f64,
    pub five: f64,
    pub fifteen: f64,
    pub host_uuid: String,
    pub created_at: chrono::NaiveDateTime,
}

impl LoadAvg {
    /// Return a Vector of LoadAvg
    /// # Params
    /// * `conn` - The r2d2 connection needed to fetch the data from the db
    /// * `uuid` - The host's uuid we want to get LoadAvg of
    /// * `size` - The number of elements to fetch
    /// * `page` - How many items you want to skip (page * size)
    pub fn get_data(
        conn: &ConnType,
        uuid: &str,
        size: i64,
        page: i64,
    ) -> Result<Vec<Self>, AppError> {
        Ok(dsl_loadavg
            .filter(host_uuid.eq(uuid))
            .limit(size)
            .offset(page * size)
            .order_by(created_at.desc())
            .load(conn)?)
    }

    /// Return a Vector of LoadAvg between min_date and max_date
    /// # Params
    /// * `conn` - The r2d2 connection needed to fetch the data from the db
    /// * `uuid` - The host's uuid we want to get LoadAvg of
    /// * `size` - The number of elements to fetch
    /// * `min_date` - Min timestamp for the data to be fetched
    /// * `max_date` - Max timestamp for the data to be fetched
    pub fn get_data_dated(
        conn: &ConnType,
        uuid: &str,
        size: i64,
        min_date: chrono::NaiveDateTime,
        max_date: chrono::NaiveDateTime,
    ) -> Result<Vec<LoadAvgDTORaw>, AppError> {
        let granularity = get_granularity(size);
        if granularity <= 1 {
            Ok(dsl_loadavg
                .select((one, five, fifteen, created_at))
                .filter(
                    host_uuid
                        .eq(uuid)
                        .and(created_at.gt(min_date).and(created_at.le(max_date))),
                )
                .limit(size)
                .order_by(created_at.desc())
                .load(conn)?)
        } else {
            Ok(sql_query(
                "
                WITH s AS 
                    (SELECT one, five, fifteen, created_at as time 
                        FROM loadavg 
                        WHERE host_uuid=$1 
                        ORDER BY created_at 
                        DESC LIMIT $2
                    ) 
                SELECT 
                    avg(one)::int8 as one, 
                    avg(five)::int8 as five, 
                    avg(fifteen)::int8 as fifteen, 
                    time::date + 
                        (extract(hour from time)::int)* '1h'::interval + 
                        (extract(minute from time)::int)* '1m'::interval + 
                        (extract(second from time)::int/$3)* '$3s'::interval as created_at 
                    FROM s 
                    GROUP BY created_at 
                    ORDER BY created_at DESC",
            )
            .bind::<Text, _>(uuid)
            .bind::<Int8, _>(size / 5) // divide by 5 because loadavg is gathered once every 5s minimum
            .bind::<Int8, _>(granularity as i64)
            .load(conn)?)
        }
    }
}

#[derive(Queryable, QueryableByName, Serialize)]
#[table_name = "loadavg"]
pub struct LoadAvgDTORaw {
    pub one: f64,
    pub five: f64,
    pub fifteen: f64,
    pub created_at: chrono::NaiveDateTime,
}

// ================
// Insertable model
// ================
#[derive(Insertable)]
#[table_name = "loadavg"]
pub struct LoadAvgDTO<'a> {
    pub one: f64,
    pub five: f64,
    pub fifteen: f64,
    pub host_uuid: &'a str,
    pub created_at: chrono::NaiveDateTime,
}

impl<'a> From<&'a HttpPostHost> for Option<LoadAvgDTO<'a>> {
    fn from(item: &'a HttpPostHost) -> Option<LoadAvgDTO<'a>> {
        let load_avg = item.load_avg.as_ref()?;
        Some(LoadAvgDTO {
            one: load_avg.one,
            five: load_avg.five,
            fifteen: load_avg.fifteen,
            host_uuid: &item.uuid,
            created_at: item.created_at,
        })
    }
}
