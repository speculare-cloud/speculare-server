use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct HostUuid {
    pub uuid: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PagedInfo {
    pub size: Option<i64>,
    pub page: Option<i64>,
}
