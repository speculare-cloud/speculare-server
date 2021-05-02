use serde::{Deserialize, Serialize};

// ====================
// Http specific struct
// Meaning those are used whenever
// there is a POST request
// ====================
#[derive(Debug, Serialize, Deserialize)]
pub struct HttpCpuStats {
    pub interrupts: i64,
    pub ctx_switches: i64,
    pub soft_interrupts: i64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HttpCpuTimes {
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
pub struct HttpLoadAvg {
    pub one: f64,
    pub five: f64,
    pub fifteen: f64,
}

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
    pub read_count: i64,
    pub read_bytes: i64,
    pub write_count: i64,
    pub write_bytes: i64,
    pub busy_time: i64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HttpMemory {
    pub total: i64,
    pub free: i64,
    pub used: i64,
    pub shared: i64,
    pub buffers: i64,
    pub cached: i64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HttpSwap {
    pub total: i64,
    pub free: i64,
    pub used: i64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HttpIoCounters {
    pub interface: String,
    pub rx_bytes: i64,
    pub rx_packets: i64,
    pub rx_errs: i64,
    pub rx_drop: i64,
    pub tx_bytes: i64,
    pub tx_packets: i64,
    pub tx_errs: i64,
    pub tx_drop: i64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HttpPostHost {
    pub uuid: String,
    pub system: String,
    pub os_version: String,
    pub hostname: String,
    pub uptime: i64,
    pub cpu_stats: Option<HttpCpuStats>,
    pub cpu_times: Option<HttpCpuTimes>,
    pub load_avg: Option<HttpLoadAvg>,
    pub disks: Option<Vec<HttpDisks>>,
    pub iostats: Option<Vec<HttpIoStats>>,
    pub memory: Option<HttpMemory>,
    pub swap: Option<HttpSwap>,
    pub iocounters: Option<Vec<HttpIoCounters>>,
    pub created_at: chrono::NaiveDateTime,
}
