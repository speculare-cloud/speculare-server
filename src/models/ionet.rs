use crate::errors::AppError;
use crate::ConnType;

use super::schema::ionets;
use super::schema::ionets::dsl::{
    created_at, host_uuid, interface, ionets as dsl_ionets, rx_bytes, tx_bytes,
};
use super::{get_granularity, get_query_range_values, Host, HttpPostHost};

use diesel::{
    pg::expression::extensions::IntervalDsl,
    sql_types::{Int8, Interval, Text},
    *,
};
use serde::{Deserialize, Serialize};

// ========================
// DATABASE Specific struct
// ========================
#[derive(Identifiable, Queryable, Debug, Serialize, Deserialize, Associations)]
#[belongs_to(Host, foreign_key = "host_uuid")]
#[table_name = "ionets"]
pub struct IoNet {
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

impl IoNet {
    /// Return a Vector of IoNet
    /// # Params
    /// * `conn` - The r2d2 connection needed to fetch the data from the db
    /// * `uuid` - The host's uuid we want to get IoNet of
    /// * `size` - The number of elements to fetch
    /// * `page` - How many items you want to skip (page * size)
    pub fn get_data(
        conn: &ConnType,
        uuid: &str,
        size: i64,
        page: i64,
    ) -> Result<Vec<Self>, AppError> {
        Ok(dsl_ionets
            .filter(host_uuid.eq(uuid))
            .limit(size)
            .offset(page * size)
            .order_by(created_at.desc())
            .load(conn)?)
    }

    /// Return a Vector of IoNet between min_date and max_date
    /// # Params
    /// * `conn` - The r2d2 connection needed to fetch the data from the db
    /// * `uuid` - The host's uuid we want to get IoNet of
    /// * `size` - The number of elements to fetch
    /// * `min_date` - Min timestamp for the data to be fetched
    /// * `max_date` - Max timestamp for the data to be fetched
    pub fn get_data_dated(
        conn: &ConnType,
        uuid: &str,
        size: i64,
        min_date: chrono::NaiveDateTime,
        max_date: chrono::NaiveDateTime,
    ) -> Result<Vec<IoNetDTORaw>, AppError> {
        let granularity = get_granularity(size);
        if granularity <= 1 {
            Ok(dsl_ionets
                .select((interface, rx_bytes, tx_bytes, created_at))
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

            // Dummy require to ensure no issue if table name change.
            // If the table's name is to be changed, we have to change it from the sql_query below.
            {
                #[allow(unused_imports)]
                use super::schema::ionets;
            }

            // Prepare and run the query
            Ok(sql_query(
                "
                WITH s AS 
                    (SELECT interface, rx_bytes, tx_bytes, created_at as time 
                        FROM ionets 
                        WHERE host_uuid=$1 
                        ORDER BY created_at 
                        DESC LIMIT $2
                    ) 
                SELECT 
                    interface, 
                    avg(rx_bytes)::int8 as rx_bytes, 
                    avg(tx_bytes)::int8 as tx_bytes, 
                    time::date + 
                        (extract(hour from time)::int)* '1h'::interval +
                        (extract(minute from time)::int/$3)* $4 +
                        (extract(second from time)::int/$5)* $6 as created_at 
                    FROM s 
                    GROUP BY created_at,interface 
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

    /// Return the numbers of IoNet the host have
    /// # Params
    /// * `conn` - The r2d2 connection needed to fetch the data from the db
    /// * `uuid` - The host's uuid we want to get the number of IoNet of
    /// * `size` - The number of elements to fetch
    pub fn count(conn: &ConnType, uuid: &str, size: i64) -> Result<i64, AppError> {
        // Dummy require to ensure no issue if table name change.
        // If the table's name is to be changed, we have to change it from the sql_query below.
        {
            #[allow(unused_imports)]
            use super::schema::ionets;
        }

        let res = sql_query(
            "
            WITH s AS 
                (SELECT id, interface, created_at 
                    FROM ionets 
                    WHERE host_uuid=$1 
                    ORDER BY created_at 
                    DESC LIMIT $2
                ) 
            SELECT 
                COUNT(DISTINCT interface) 
                FROM s",
        )
        .bind::<Text, _>(uuid)
        .bind::<Int8, _>(size)
        .load::<IoNetCount>(conn)?;

        if res.is_empty() {
            Ok(0)
        } else {
            Ok(res[0].count)
        }
    }
}

#[derive(Queryable, QueryableByName, Serialize)]
#[table_name = "ionets"]
pub struct IoNetDTORaw {
    pub interface: String,
    pub rx_bytes: i64,
    pub tx_bytes: i64,
    pub created_at: chrono::NaiveDateTime,
}

#[derive(Queryable, QueryableByName, Serialize)]
pub struct IoNetCount {
    #[sql_type = "Int8"]
    pub count: i64,
}

// ================
// Insertable model
// ================
#[derive(Insertable)]
#[table_name = "ionets"]
pub struct IoNetDTO<'a> {
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

pub type IoNetDTOList<'a> = Vec<IoNetDTO<'a>>;
impl<'a> From<&'a HttpPostHost> for Option<IoNetDTOList<'a>> {
    fn from(item: &'a HttpPostHost) -> Option<IoNetDTOList<'a>> {
        let ionets = item.ionets.as_ref()?;
        let mut list = Vec::with_capacity(ionets.len());
        for iocounter in ionets {
            list.push(IoNetDTO {
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
