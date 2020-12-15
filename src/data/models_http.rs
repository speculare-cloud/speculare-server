use crate::data::models_db::*;

use serde::{Deserialize, Serialize};

// HTTP Specific struct
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
pub struct HttpMemory {
    pub total_virt: u64,
    pub avail_virt: u64,
    pub total_swap: u64,
    pub avail_swap: u64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HttpPostData {
    pub uuid: String,
    pub os: String,
    pub hostname: String,
    pub uptime: i64,
    pub cpu_freq: i64,
    pub load_avg: HttpLoadAvg,
    pub disks: Vec<HttpDisks>,
    //pub user: Option<Vec<String>>,
    pub memory: HttpMemory,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HttpGetData {
    pub os: String,
    pub hostname: String,
    pub uptime: i64,
    pub uuid: String,
    pub cpu_freq: CpuInfo,
    pub load_avg: LoadAvg,
    pub disks: Disks,
    pub memory: Memory,
    //pub user: String,
}
