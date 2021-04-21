use serde::{Deserialize, Serialize};

pub mod hosts;
pub use hosts::*;

pub mod cpu;
pub use cpu::*;

pub mod disks;
pub use disks::*;

pub mod memory;
pub use memory::*;

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
