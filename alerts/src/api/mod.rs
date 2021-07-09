use serde::{Deserialize, Serialize};

pub mod alerts;

#[derive(Debug, Serialize, Deserialize)]
pub struct PagedInfo {
    pub size: Option<i64>,
    pub page: Option<i64>,
}
