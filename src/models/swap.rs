use crate::errors::AppError;
use crate::ConnType;

use super::schema::swap;
use super::schema::swap::dsl::{created_at, free, host_uuid, swap as dsl_swap, total, used};
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
#[table_name = "swap"]
pub struct Swap {
    pub id: i64,
    pub total: i64,
    pub free: i64,
    pub used: i64,
    pub host_uuid: String,
    pub created_at: chrono::NaiveDateTime,
}

impl Swap {
    /// Return a Vector of Swap
    /// # Params
    /// * `conn` - The r2d2 connection needed to fetch the data from the db
    /// * `uuid` - The host's uuid we want to get Swap of
    /// * `size` - The number of elements to fetch
    /// * `page` - How many items you want to skip (page * size)
    pub fn get_data(
        conn: &ConnType,
        uuid: &str,
        size: i64,
        page: i64,
    ) -> Result<Vec<Self>, AppError> {
        Ok(dsl_swap
            .filter(host_uuid.eq(uuid))
            .limit(size)
            .offset(page * size)
            .order_by(created_at.desc())
            .load(conn)?)
    }

    /// Return a Vector of Swap between min_date and max_date
    /// # Params
    /// * `conn` - The r2d2 connection needed to fetch the data from the db
    /// * `uuid` - The host's uuid we want to get Swap of
    /// * `size` - The number of elements to fetch
    /// * `min_date` - Min timestamp for the data to be fetched
    /// * `max_date` - Max timestamp for the data to be fetched
    pub fn get_data_dated(
        conn: &ConnType,
        uuid: &str,
        size: i64,
        min_date: chrono::NaiveDateTime,
        max_date: chrono::NaiveDateTime,
    ) -> Result<Vec<SwapDTORaw>, AppError> {
        let granularity = get_granularity(size);
        if granularity <= 1 {
            Ok(dsl_swap
                .select((total, free, used, created_at))
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
                    (SELECT total, free, used, created_at as time 
                        FROM swap 
                        WHERE host_uuid=$1 
                        ORDER BY created_at 
                        DESC LIMIT $2
                    ) 
                SELECT 
                    avg(total)::int8 as total, 
                    avg(free)::int8 as free, 
                    avg(used)::int8 as used, 
                    time::date + 
                        (extract(hour from time)::int)* '1h'::interval +
                        (extract(minute from time)::int/$3)* '$3m$4s'::interval +
                        (extract(second from time)::int/$5)* '$5s'::interval as created_at 
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
#[table_name = "swap"]
pub struct SwapDTORaw {
    pub total: i64,
    pub free: i64,
    pub used: i64,
    pub created_at: chrono::NaiveDateTime,
}

// ================
// Insertable model
// ================
#[derive(Insertable)]
#[table_name = "swap"]
pub struct SwapDTO<'a> {
    pub total: i64,
    pub free: i64,
    pub used: i64,
    pub host_uuid: &'a str,
    pub created_at: chrono::NaiveDateTime,
}

impl<'a> From<&'a HttpPostHost> for Option<SwapDTO<'a>> {
    fn from(item: &'a HttpPostHost) -> Option<SwapDTO<'a>> {
        let swap = item.swap.as_ref()?;
        Some(SwapDTO {
            total: swap.total as i64,
            free: swap.free as i64,
            used: swap.used as i64,
            host_uuid: &item.uuid,
            created_at: item.created_at,
        })
    }
}
