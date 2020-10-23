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
pub struct SData {
    pub os: String,
    pub hostname: String,
    pub uptime: i64,
    pub uuid: String,
    pub cpu_freq: i64,
    pub sensors: Vec<SSensors>,
    pub disks: Vec<SDisks>,
    pub user: String,
    pub mac_address: String,
}
