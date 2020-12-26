use super::schema::*;
use super::{Host, HttpPostHost};

use diesel::*;
use serde::{Deserialize, Serialize};

// DATABASE Specific struct
#[derive(Identifiable, Queryable, Debug, Serialize, Deserialize, Associations)]
#[belongs_to(Host, foreign_key = "host_uuid")]
#[table_name = "load_avg"]
pub struct LoadAvg {
    pub id: i32,
    pub one: f64,
    pub five: f64,
    pub fifteen: f64,
    pub host_uuid: String,
    pub created_at: chrono::NaiveDateTime,
}

// Insertable models
#[derive(Insertable)]
#[table_name = "load_avg"]
pub struct NewLoadAvg<'a> {
    pub one: f64,
    pub five: f64,
    pub fifteen: f64,
    pub host_uuid: &'a str,
    pub created_at: chrono::NaiveDateTime,
}

impl<'a> From<&'a HttpPostHost> for NewLoadAvg<'a> {
    fn from(item: &'a HttpPostHost) -> NewLoadAvg<'a> {
        NewLoadAvg {
            one: item.load_avg.one,
            five: item.load_avg.five,
            fifteen: item.load_avg.fifteen,
            host_uuid: &item.uuid,
            created_at: item.created_at,
        }
    }
}
