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
pub struct CpuStats {
    pub user: i64,
    pub nice: i64,
    pub system: i64,
    pub idle: i64,
    pub iowait: i64,
    pub irq: i64,
    pub softirq: i64,
    pub steal: i64,
    pub guest: i64,
    pub guest_nice: i64,
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
    pub system: String,
    pub os_version: String,
    pub hostname: String,
    pub uptime: i64,
    pub cpu_stats: Option<CpuStats>,
    pub load_avg: Option<HttpLoadAvg>,
    pub disks: Option<Vec<HttpDisks>>,
    pub iostats: Option<Vec<HttpIoStats>>,
    pub memory: Option<HttpMemory>,
    pub created_at: chrono::NaiveDateTime,
}
