use super::{CpuInfo, Disks, IoStats, LoadAvg, Memory};

use serde::{Deserialize, Serialize};

// ====================
// Http specific struct
// Meaning those are used whenever
// there is a POST/GET request
// ====================

#[derive(Debug, Serialize, Deserialize)]
pub struct HttpDisks {
    pub name: String,
    pub mount_point: String,
    pub total_space: i64,
    pub avail_space: i64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HttpIoStats {
    pub device_name: String,
    pub bytes_read: i64,
    pub bytes_wrtn: i64,
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
pub struct HttpPostHost {
    pub uuid: String,
    pub os: String,
    pub hostname: String,
    pub uptime: i64,
    pub cpu_freq: i64,
    pub load_avg: Option<HttpLoadAvg>,
    pub disks: Option<Vec<HttpDisks>>,
    pub iostats: Option<Vec<HttpIoStats>>,
    pub memory: Option<HttpMemory>,
    pub created_at: chrono::NaiveDateTime,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HttpGetHost {
    pub uuid: String,
    pub os: String,
    pub hostname: String,
    pub uptime: i64,
    pub cpu_freq: CpuInfo,
    pub load_avg: Option<LoadAvg>,
    pub disks: Option<Disks>,
    pub iostats: Option<IoStats>,
    pub memory: Option<Memory>,
}
