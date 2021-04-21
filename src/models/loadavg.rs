use crate::errors::AppError;
use crate::ConnType;

use super::schema::load_avg::dsl::{created_at, host_uuid, load_avg as dsl_loadavg};
use super::schema::*;
use super::{Host, HttpPostHost};

use diesel::*;
use serde::{Deserialize, Serialize};

// ========================
// DATABASE Specific struct
// ========================
#[derive(Identifiable, Queryable, Debug, Serialize, Deserialize, Associations)]
#[belongs_to(Host, foreign_key = "host_uuid")]
#[table_name = "load_avg"]
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
        min_date: Option<chrono::NaiveDateTime>,
        max_date: Option<chrono::NaiveDateTime>,
    ) -> Result<Vec<Self>, AppError> {
        if min_date.is_some() && max_date.is_some() {
            Ok(dsl_loadavg
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
            Ok(dsl_loadavg
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
#[table_name = "load_avg"]
pub struct NewLoadAvg<'a> {
    pub one: f64,
    pub five: f64,
    pub fifteen: f64,
    pub host_uuid: &'a str,
    pub created_at: chrono::NaiveDateTime,
}

impl<'a> From<&'a HttpPostHost> for Option<NewLoadAvg<'a>> {
    fn from(item: &'a HttpPostHost) -> Option<NewLoadAvg<'a>> {
        let load_avg = item.load_avg.as_ref()?;
        Some(NewLoadAvg {
            one: load_avg.one,
            five: load_avg.five,
            fifteen: load_avg.fifteen,
            host_uuid: &item.uuid,
            created_at: item.created_at,
        })
    }
}
