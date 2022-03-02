use crate::errors::AppError;
use crate::ConnType;

use crate::models::schema::disks;
use crate::models::schema::disks::dsl::{
    avail_space, created_at, disk_name, disks as dsl_disks, host_uuid, total_space,
};
use crate::models::{get_granularity, HttpPostHost};

use diesel::{
    sql_types::{Int8, Text, Timestamp},
    *,
};
use serde::{Deserialize, Serialize};

/// DB Specific struct for disks table
#[derive(Identifiable, Queryable, Debug, Serialize, Deserialize)]
#[table_name = "disks"]
pub struct Disk {
    pub id: i64,
    pub disk_name: String,
    pub mount_point: String,
    pub total_space: i64,
    pub avail_space: i64,
    pub host_uuid: String,
    pub created_at: chrono::NaiveDateTime,
}

impl Disk {
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
        min_date: chrono::NaiveDateTime,
        max_date: chrono::NaiveDateTime,
    ) -> Result<Vec<DiskDTORaw>, AppError> {
        let size = (max_date - min_date).num_seconds();
        let granularity = get_granularity(size);
        if granularity <= 1 {
            Ok(dsl_disks
                .select((disk_name, total_space, avail_space, created_at))
                .filter(
                    host_uuid
                        .eq(uuid)
                        .and(created_at.gt(min_date).and(created_at.le(max_date))),
                )
                // size * 10 as workaround for the moment
                // TODO - Size * by the number of disks in the system
                .limit(size * 10)
                .order_by(created_at.desc())
                .load(conn)?)
        } else {
            // Dummy require to ensure no issue if table name change.
            // If the table's name is to be changed, we have to change it from the sql_query below.
            {
                #[allow(unused_imports)]
                use crate::models::schema::disks;
            }

            // Prepare and run the query
            Ok(sql_query(format!(
                "
                WITH s AS (
                    SELECT 
                        disk_name, 
                        avg(total_space)::int8 as total_space, 
                        avg(avail_space)::int8 as avail_space,
                        time_bucket('{}s', created_at) as time 
                    FROM disks 
                    WHERE host_uuid=$1 AND created_at BETWEEN $2 AND $3 
                    GROUP BY time,disk_name ORDER BY time DESC
                )
                SELECT 
                    disk_name,
                    total_space,
                    avail_space,
                    time as created_at
                FROM s",
                granularity
            ))
            .bind::<Text, _>(uuid)
            .bind::<Timestamp, _>(min_date)
            .bind::<Timestamp, _>(max_date)
            .load(conn)?)
        }
    }

    /// Return the numbers of disks the host have
    /// # Params
    /// * `conn` - The r2d2 connection needed to fetch the data from the db
    /// * `uuid` - The host's uuid we want to get the number of disks of
    /// * `size` - The number of elements to fetch
    pub fn count(conn: &ConnType, uuid: &str, size: i64) -> Result<i64, AppError> {
        // Dummy require to ensure no issue if table name change.
        // If the table's name is to be changed, we have to change it from the sql_query below.
        {
            #[allow(unused_imports)]
            use crate::models::schema::disks;
        }

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

        if res.is_empty() {
            Ok(0)
        } else {
            Ok(res[0].count)
        }
    }
}

#[derive(Queryable, QueryableByName, Serialize)]
#[table_name = "disks"]
pub struct DiskDTORaw {
    pub disk_name: String,
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
pub struct DiskDTO<'a> {
    pub disk_name: &'a str,
    pub mount_point: &'a str,
    pub total_space: i64,
    pub avail_space: i64,
    pub host_uuid: &'a str,
    pub created_at: chrono::NaiveDateTime,
}

pub type DiskDTOList<'a> = Vec<DiskDTO<'a>>;
impl<'a> From<&'a HttpPostHost> for Option<DiskDTOList<'a>> {
    fn from(item: &'a HttpPostHost) -> Option<DiskDTOList<'a>> {
        let disks = item.disks.as_ref()?;
        let mut list = Vec::with_capacity(disks.len());
        for disk in disks {
            list.push(DiskDTO {
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
