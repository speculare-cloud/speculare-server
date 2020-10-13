use crate::schema::*;

use diesel::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct SSensors {
    pub label: String,
    pub temp: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SData {
    pub os: String,
    pub hostname: String,
    pub uptime: i64,
    pub uuid: String,
    pub cpu_freq: i64,
    pub sensors: Vec<SSensors>,
    pub user: String,
    pub mac_address: String,
}

/* DATABASE Specific struct */
#[derive(Identifiable, Queryable, Debug, Serialize, Deserialize)]
#[table_name = "sensors"]
pub struct Sensors {
    pub id: i32,
    pub label: String,
    pub temp: f64,
}

#[derive(Insertable)]
#[table_name = "sensors"]
pub struct NewSensors<'a> {
    pub label: &'a str,
    pub temp: f64,
}

#[derive(Identifiable, Queryable, Debug, Serialize, Deserialize)]
#[table_name = "data"]
pub struct Data {
    pub id: i32,
    pub os: String,
    pub hostname: String,
    pub uptime: i64,
    pub uuid: String,
    pub cpu_freq: i64,
    pub active_user: String,
    pub mac_address: String,
    pub created_at: chrono::NaiveDateTime,
}

#[derive(Insertable)]
#[table_name = "data"]
pub struct NewData<'a> {
    pub os: &'a str,
    pub hostname: &'a str,
    pub uptime: i64,
    pub uuid: &'a str,
    pub cpu_freq: i64,
    pub active_user: &'a str,
    pub mac_address: &'a str,
    pub created_at: chrono::NaiveDateTime,
}

#[derive(Identifiable, Queryable, Debug, Associations, Serialize, Deserialize)]
#[belongs_to(Data)]
#[belongs_to(Sensors)]
#[table_name = "datasensors"]
pub struct DataSensors {
    pub id: i32,
    pub data_id: i32,
    pub sensors_id: i32,
}

#[derive(Insertable)]
#[table_name = "datasensors"]
pub struct NewDataSensors {
    pub data_id: i32,
    pub sensors_id: i32,
}
