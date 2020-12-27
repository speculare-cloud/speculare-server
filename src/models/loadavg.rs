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
    pub id: i32,
    pub one: f64,
    pub five: f64,
    pub fifteen: f64,
    pub host_uuid: String,
    pub created_at: chrono::NaiveDateTime,
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