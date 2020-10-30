use crate::models_db::*;

use serde::{Deserialize, Serialize};

// HTTP Specific struct
#[derive(Debug, Serialize, Deserialize)]
pub struct SSensors {
    pub label: String,
    pub temp: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SDisks {
    pub name: String,
    pub mount_point: String,
    pub total_space: i64,
    pub avail_space: i64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SLoadAvg {
    pub one: f64,
    pub five: f64,
    pub fifteen: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SData {
    pub os: String,
    pub hostname: String,
    pub uptime: i64,
    pub uuid: String,
    pub cpu_freq: i64,
    pub load_avg: SLoadAvg,
    pub sensors: Vec<SSensors>,
    pub disks: Vec<SDisks>,
    pub user: String,
    pub mac_address: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RData {
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
