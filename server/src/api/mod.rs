//! In the API mod, we're often using web::block to offload
//! synchronous operation (access to Diesel's conns) allowing
//! Actix to handle another request while the sync task is
//! being performed.

use serde::{Deserialize, Serialize};
use sproot::errors::{AppError, AppErrorType};

pub mod balerts;
pub mod cpustats;
pub mod cputimes;
pub mod disks;
pub mod hosts;
pub mod ioblock;
pub mod ionet;
pub mod loadavg;
pub mod memory;
pub mod swap;

#[derive(Debug, Serialize, Deserialize)]
pub struct PagedInfo {
    pub uuid: String,
    pub size: Option<i64>,
    pub page: Option<i64>,
    pub min_date: Option<chrono::NaiveDateTime>,
    pub max_date: Option<chrono::NaiveDateTime>,
}

impl PagedInfo {
    pub fn is_dated(&self) -> bool {
        self.min_date.is_some() && self.max_date.is_some()
    }

    pub fn get_size_page(&self) -> Result<(i64, i64), AppError> {
        match (self.size, self.page) {
            p if p.0.is_some()
                && p.0.unwrap() > 0
                && p.0.unwrap() < 5000
                && p.1.is_some()
                && p.1.unwrap() >= 0 =>
            {
                Ok((p.0.unwrap(), p.1.unwrap()))
            }
            _ => Err(AppError {
                message: Some("The parameters are incorrect".to_owned()),
                cause: Some("Size must be > 0 && < 5000 and Page must be >= 0".to_owned()),
                error_type: AppErrorType::InvalidRequest,
            }),
        }
    }

    pub fn get_size(&self) -> Result<i64, AppError> {
        match self.size {
            s if s.is_some() && s.unwrap() > 0 && s.unwrap() < 5000 => Ok(s.unwrap()),
            _ => Err(AppError {
                message: Some("The parameters are incorrect".to_owned()),
                cause: Some("Size must be > 0".to_owned()),
                error_type: AppErrorType::InvalidRequest,
            }),
        }
    }
}
