use crate::data::schema::*;

use diesel::*;
use serde::{Deserialize, Serialize};

// DATABASE Specific struct
#[derive(Identifiable, Queryable, Debug, Serialize, Deserialize, Associations)]
#[belongs_to(Host, foreign_key = "host_uuid")]
#[table_name = "disks"]
pub struct Disks {
    pub id: i32,
    pub disk_name: String,
    pub mount_point: String,
    pub total_space: i64,
    pub avail_space: i64,
    pub host_uuid: String,
    pub created_at: chrono::NaiveDateTime,
}

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

#[derive(Identifiable, Queryable, Debug, Serialize, Deserialize, Associations)]
#[belongs_to(Host, foreign_key = "host_uuid")]
#[table_name = "cpu_info"]
pub struct CpuInfo {
    pub id: i32,
    pub cpu_freq: i64,
    pub host_uuid: String,
    pub created_at: chrono::NaiveDateTime,
}

#[derive(Identifiable, Queryable, Debug, Serialize, Deserialize, Associations)]
#[belongs_to(Host, foreign_key = "host_uuid")]
#[table_name = "memory"]
pub struct Memory {
    pub id: i32,
    pub total_virt: i64,
    pub avail_virt: i64,
    pub total_swap: i64,
    pub avail_swap: i64,
    pub host_uuid: String,
    pub created_at: chrono::NaiveDateTime,
}

#[derive(Identifiable, Queryable, Debug, Serialize, Deserialize, Insertable, AsChangeset)]
#[table_name = "hosts"]
#[primary_key(uuid)]
pub struct Host {
    pub os: String,
    pub hostname: String,
    pub uptime: i64,
    pub uuid: String,
    pub created_at: chrono::NaiveDateTime,
}

// Insertable models
#[derive(Insertable)]
#[table_name = "disks"]
pub struct NewDisks<'a> {
    pub disk_name: &'a str,
    pub mount_point: &'a str,
    pub total_space: i64,
    pub avail_space: i64,
    pub host_uuid: &'a str,
    pub created_at: chrono::NaiveDateTime,
}

#[derive(Insertable)]
#[table_name = "load_avg"]
pub struct NewLoadAvg<'a> {
    pub one: f64,
    pub five: f64,
    pub fifteen: f64,
    pub host_uuid: &'a str,
    pub created_at: chrono::NaiveDateTime,
}

#[derive(Insertable)]
#[table_name = "cpu_info"]
pub struct NewCpuInfo<'a> {
    pub cpu_freq: i64,
    pub host_uuid: &'a str,
    pub created_at: chrono::NaiveDateTime,
}

#[derive(Insertable)]
#[table_name = "memory"]
pub struct NewMemory<'a> {
    pub total_virt: i64,
    pub avail_virt: i64,
    pub total_swap: i64,
    pub avail_swap: i64,
    pub host_uuid: &'a str,
    pub created_at: chrono::NaiveDateTime,
}
