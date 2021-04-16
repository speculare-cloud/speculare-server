pub mod hosts;
pub use hosts::*;

pub mod cpu;
pub use cpu::*;

use serde::{Deserialize, Serialize};

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
}
