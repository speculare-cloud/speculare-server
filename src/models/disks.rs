use crate::errors::AppError;
use crate::ConnType;

use super::schema::disks;
use super::schema::disks::dsl::{
    avail_space, created_at, disk_name, disks as dsl_disks, host_uuid, total_space,
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
#[table_name = "disks"]
pub struct Disks {
    pub id: i64,
    pub disk_name: String,
    pub mount_point: String,
    pub total_space: i64,
    pub avail_space: i64,
    pub host_uuid: String,
    pub created_at: chrono::NaiveDateTime,
}

impl Disks {
    /// Return a Vector of Disks
    /// # Params
    /// * `conn` - The r2d2 connection needed to fetch the data from the db
    /// * `uuid` - The host's uuid we want to get Disks of
    /// * `size` - The number of elements to fetch
    /// * `page` - How many items you want to skip (page * size)
    pub fn get_data(
        conn: &ConnType,
        uuid: &str,
        size: i64,
        page: i64,
    ) -> Result<Vec<Self>, AppError> {
        Ok(dsl_disks
            .filter(host_uuid.eq(uuid))
            .limit(size)
            .offset(page * size)
            .order_by(created_at.desc())
            .load(conn)?)
    }

    /// Return a Vector of Disks between min_date and max_date
    /// # Params
    /// * `conn` - The r2d2 connection needed to fetch the data from the db
    /// * `uuid` - The host's uuid we want to get Disks of
    /// * `size` - The number of elements to fetch
    /// * `min_date` - Min timestamp for the data to be fetched
    /// * `max_date` - Max timestamp for the data to be fetched
    pub fn get_data_dated(
        conn: &ConnType,
        uuid: &str,
        size: i64,
        min_date: chrono::NaiveDateTime,
        max_date: chrono::NaiveDateTime,
    ) -> Result<Vec<DisksDTORaw>, AppError> {
        let granularity = get_granularity(size);
        if granularity <= 1 {
            Ok(dsl_disks
                .select((disk_name, total_space, avail_space, created_at))
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
                    (SELECT disk_name, total_space, avail_space, created_at as time 
                        FROM disks 
                        WHERE host_uuid=$1 
                        ORDER BY created_at 
                        DESC LIMIT $2
                    )
                SELECT 
                    disk_name, 
                    avg(total_space)::int8 as total_space, 
                    avg(avail_space)::int8 as avail_space, 
                    time::date + 
                        (extract(hour from time)::int)* '1h'::interval +
                        (extract(minute from time)::int/$3)* $4 +
                        (extract(second from time)::int/$5)* $6 as created_at 
                    FROM s 
                    GROUP BY created_at,disk_name 
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

    /// Return the numbers of disks the host have
    /// # Params
    /// * `conn` - The r2d2 connection needed to fetch the data from the db
    /// * `uuid` - The host's uuid we want to get the number of disks of
    /// * `size` - The number of elements to fetch
    pub fn count(conn: &ConnType, uuid: &str, size: i64) -> Result<i64, AppError> {
        let res = sql_query(
            "
            WITH s AS 
                (SELECT id, disk_name, created_at 
                    FROM disks 
                    WHERE host_uuid=$1 
                    ORDER BY created_at 
                    DESC LIMIT $2
                ) 
            SELECT 
                COUNT(DISTINCT disk_name) 
                FROM s",
        )
        .bind::<Text, _>(uuid)
        .bind::<Int8, _>(size)
        .load::<DisksCount>(conn)?;

        if res.len() == 0 {
            Ok(0)
        } else {
            Ok(res[0].count)
        }
    }
}

#[derive(Queryable, QueryableByName, Serialize)]
#[table_name = "disks"]
pub struct DisksDTORaw {
    pub disk_name: String,
    // pub mount_point: String,
    pub total_space: i64,
    pub avail_space: i64,
    pub created_at: chrono::NaiveDateTime,
}

#[derive(Queryable, QueryableByName, Serialize)]
pub struct DisksCount {
    #[sql_type = "Int8"]
    pub count: i64,
}

// ================
// Insertable model
// ================
#[derive(Insertable)]
#[table_name = "disks"]
pub struct DisksDTO<'a> {
    pub disk_name: &'a str,
    pub mount_point: &'a str,
    pub total_space: i64,
    pub avail_space: i64,
    pub host_uuid: &'a str,
    pub created_at: chrono::NaiveDateTime,
}

pub type DisksDTOList<'a> = Vec<DisksDTO<'a>>;
impl<'a> From<&'a HttpPostHost> for Option<DisksDTOList<'a>> {
    fn from(item: &'a HttpPostHost) -> Option<DisksDTOList<'a>> {
        let disks = item.disks.as_ref()?;
        let mut list = Vec::with_capacity(disks.len());
        for disk in disks {
            list.push(DisksDTO {
                disk_name: &disk.name,
                mount_point: &disk.mount_point,
                total_space: disk.total_space,
                avail_space: disk.avail_space,
                host_uuid: &item.uuid,
                created_at: item.created_at,
            })
        }
        Some(list)
    }
}
