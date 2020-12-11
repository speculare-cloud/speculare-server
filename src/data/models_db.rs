use crate::data::schema::*;

use diesel::*;
use serde::{Deserialize, Serialize};

// DATABASE Specific struct
#[derive(Identifiable, Queryable, Debug, Serialize, Deserialize, Associations)]
#[belongs_to(Data, foreign_key = "data_uuid")]
#[table_name = "sensors"]
pub struct Sensors {
    pub id: i32,
    pub label: String,
    pub temp: f64,
    pub data_uuid: String,
    pub created_at: chrono::NaiveDateTime,
}

#[derive(Identifiable, Queryable, Debug, Serialize, Deserialize, Associations)]
#[belongs_to(Data, foreign_key = "data_uuid")]
#[table_name = "disks"]
pub struct Disks {
    pub id: i32,
    pub disk_name: String,
    pub mount_point: String,
    pub total_space: i64,
    pub avail_space: i64,
    pub data_uuid: String,
    pub created_at: chrono::NaiveDateTime,
}

#[derive(Identifiable, Queryable, Debug, Serialize, Deserialize, Associations)]
#[belongs_to(Data, foreign_key = "data_uuid")]
#[table_name = "load_avg"]
pub struct LoadAvg {
    pub id: i32,
    pub one: f64,
    pub five: f64,
    pub fifteen: f64,
    pub data_uuid: String,
    pub created_at: chrono::NaiveDateTime,
}

#[derive(Identifiable, Queryable, Debug, Serialize, Deserialize, Associations)]
#[belongs_to(Data, foreign_key = "data_uuid")]
#[table_name = "cpu_info"]
pub struct CpuInfo {
    pub id: i32,
    pub cpu_freq: i64,
    pub data_uuid: String,
    pub created_at: chrono::NaiveDateTime,
}

#[derive(Identifiable, Queryable, Debug, Serialize, Deserialize, Insertable, AsChangeset)]
#[table_name = "data"]
#[primary_key(uuid)]
pub struct Data {
    pub os: String,
    pub hostname: String,
    pub uptime: i64,
    pub uuid: String,
    pub active_user: String,
    pub mac_address: String,
    pub created_at: chrono::NaiveDateTime,
}

// Insertable models
#[derive(Insertable)]
#[table_name = "sensors"]
pub struct NewSensors<'a> {
    pub label: &'a str,
    pub temp: f64,
    pub data_uuid: &'a str,
    pub created_at: chrono::NaiveDateTime,
}

#[derive(Insertable)]
#[table_name = "disks"]
pub struct NewDisks<'a> {
    pub disk_name: &'a str,
    pub mount_point: &'a str,
    pub total_space: i64,
    pub avail_space: i64,
    pub data_uuid: &'a str,
    pub created_at: chrono::NaiveDateTime,
}

#[derive(Insertable)]
#[table_name = "load_avg"]
pub struct NewLoadAvg<'a> {
    pub one: f64,
    pub five: f64,
    pub fifteen: f64,
    pub data_uuid: &'a str,
    pub created_at: chrono::NaiveDateTime,
}

#[derive(Insertable)]
#[table_name = "cpu_info"]
pub struct NewCpuInfo<'a> {
    pub cpu_freq: i64,
    pub data_uuid: &'a str,
    pub created_at: chrono::NaiveDateTime,
}
