use serde::{Deserialize, Serialize};

pub mod cpustats;
pub mod cputimes;
pub mod disks;
pub mod hosts;
pub mod iocounters;
pub mod iostats;
pub mod loadavg;
pub mod memory;
pub mod swap;

#[derive(Debug, Serialize, Deserialize)]
pub struct PagedInfo {
    pub size: Option<i64>,
    pub page: Option<i64>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PagedInfoSpecific {
    pub uuid: String,
    pub size: Option<i64>,
    pub page: Option<i64>,
    pub min_date: Option<chrono::NaiveDateTime>,
    pub max_date: Option<chrono::NaiveDateTime>,
}
