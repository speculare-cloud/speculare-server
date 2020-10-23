use crate::schema::*;

use diesel::*;
use serde::{Deserialize, Serialize};

// DATABASE Specific struct
#[derive(Identifiable, Queryable, Debug, Serialize, Deserialize, Associations)]
#[belongs_to(Data<'__a>, foreign_key = "data_uuid")]
#[table_name = "sensors"]
pub struct Sensors {
    pub id: i32,
    pub label: String,
    pub temp: f64,
    pub data_uuid: String,
}

#[derive(Identifiable, Queryable, Debug, Serialize, Deserialize, Associations)]
#[belongs_to(Data<'__a>, foreign_key = "data_uuid")]
#[table_name = "disks"]
pub struct Disks {
    pub id: i32,
    pub disk_name: String,
    pub mount_point: String,
    pub total_space: i64,
    pub avail_space: i64,
    pub data_uuid: String,
}

#[derive(Identifiable, Queryable, Debug, Serialize, Deserialize, Insertable, AsChangeset)]
#[table_name = "data"]
#[primary_key(uuid)]
pub struct Data<'a> {
    pub os: &'a str,
    pub hostname: &'a str,
    pub uptime: i64,
    pub uuid: &'a str,
    pub cpu_freq: i64,
    pub active_user: &'a str,
    pub mac_address: &'a str,
    pub created_at: chrono::NaiveDateTime,
}

// Insertable models
#[derive(Insertable)]
#[table_name = "sensors"]
pub struct InsSensors<'a> {
    pub label: &'a str,
    pub temp: f64,
    pub data_uuid: &'a str,
}

#[derive(Insertable)]
#[table_name = "disks"]
pub struct InsDisks<'a> {
    pub disk_name: &'a str,
    pub mount_point: &'a str,
    pub total_space: i64,
    pub avail_space: i64,
    pub data_uuid: &'a str,
}
