use crate::data::models_db::*;

use serde::{Deserialize, Serialize};

// HTTP Specific struct
#[derive(Debug, Serialize, Deserialize)]
pub struct HttpSensors {
    pub label: String,
    pub temp: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HttpDisks {
    pub name: String,
    pub mount_point: String,
    pub total_space: i64,
    pub avail_space: i64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HttpLoadAvg {
    pub one: f64,
    pub five: f64,
    pub fifteen: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HttpPostData {
    pub os: String,
    pub hostname: String,
    pub uptime: i64,
    pub uuid: String,
    pub cpu_freq: i64,
    pub load_avg: HttpLoadAvg,
    pub sensors: Vec<HttpSensors>,
    pub disks: Vec<HttpDisks>,
    pub user: String,
    pub mac_address: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HttpGetData {
    pub os: String,
    pub hostname: String,
    pub uptime: i64,
    pub uuid: String,
    pub cpu_freq: Vec<CpuInfo>,
    pub load_avg: Vec<LoadAvg>,
    pub sensors: Vec<Sensors>,
    pub disks: Vec<Disks>,
    pub user: String,
    pub mac_address: String,
}
