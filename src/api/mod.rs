//! In the API mod, we're often using web::block to offload
//! synchronous operation (access to Diesel's conns) allowing
//! Actix to handle another request while the sync task is
//! being performed.
#[cfg(feature = "auth")]
use actix_session::Session;
use serde::{Deserialize, Serialize};
use sproot::errors::{AppError, AppErrorType};
#[cfg(feature = "auth")]
use uuid::Uuid;

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
pub struct Paged {
    pub size: Option<i64>,
    pub page: Option<i64>,
}

impl Paged {
    pub fn get_size_page(&self) -> Result<(i64, i64), AppError> {
        let size = self.size.unwrap_or(100);
        let page = self.page.unwrap_or(0);
        match (size, page) {
            v if v.0 > 0 && v.0 < 5000 && v.1 >= 0 => Ok((v.0, v.1)),
            _ => Err(AppError {
                message: "Size must be > 0 && < 5000 and Page must be >= 0".to_owned(),
                error_type: AppErrorType::InvalidRequest,
            }),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SpecificDated {
    pub uuid: String,
    pub min_date: chrono::NaiveDateTime,
    pub max_date: chrono::NaiveDateTime,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SpecificPaged {
    pub uuid: String,
    pub size: Option<i64>,
    pub page: Option<i64>,
}

impl SpecificPaged {
    pub fn get_size_page(&self) -> Result<(i64, i64), AppError> {
        let size = self.size.unwrap_or(100);
        let page = self.page.unwrap_or(0);
        match (size, page) {
            v if v.0 > 0 && v.0 < 5000 && v.1 >= 0 => Ok((v.0, v.1)),
            _ => Err(AppError {
                message: "Size must be > 0 && < 5000 and Page must be >= 0".to_owned(),
                error_type: AppErrorType::InvalidRequest,
            }),
        }
    }

    pub fn get_size(&self) -> Result<i64, AppError> {
        let size = self.size.unwrap_or(100);
        match size {
            s if s > 0 && s < 5000 => Ok(s),
            _ => Err(AppError {
                message: "Size must be > 0".to_owned(),
                error_type: AppErrorType::InvalidRequest,
            }),
        }
    }
}

/// Get the Uuid of the user from his Session or
/// return an InvalidToken error if not found
#[cfg(feature = "auth")]
pub fn get_user_session(session: &Session) -> Result<Uuid, AppError> {
    match session.get::<String>("user_id") {
        Ok(Some(id)) => Ok(Uuid::parse_str(&id).unwrap()),
        _ => Err(AppError {
            message: "Missing user_id in the session".to_owned(),
            error_type: AppErrorType::InvalidToken,
        }),
    }
}
