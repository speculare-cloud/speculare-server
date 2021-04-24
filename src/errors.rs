use actix_web::{error::ResponseError, http::StatusCode, HttpResponse};
use serde::Serialize;
use std::fmt;

#[derive(Debug)]
pub enum AppErrorType {
    DbError,
    PoolError,
    InvalidRequest,
    InvalidToken,
    BlockingError,
}

#[derive(Debug)]
pub struct AppError {
    pub message: Option<String>,
    pub cause: Option<String>,
    pub error_type: AppErrorType,
}

impl AppError {
    pub fn message(&self) -> String {
        match &*self {
            AppError {
                message: Some(message),
                ..
            } => message.clone(),
            AppError {
                message: None,
                error_type: AppErrorType::PoolError,
                ..
            } => "Cannot get the connection pool to the database".to_string(),
            AppError {
                message: None,
                error_type: AppErrorType::InvalidToken,
                ..
            } => "The token is invalid or has been expired".to_string(),
            AppError {
                message: None,
                error_type: AppErrorType::InvalidRequest,
                ..
            } => "Invalid request".to_string(),
            _ => "An unexpected error has occurred".to_string(),
        }
    }
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        write!(f, "{:?}", self)
    }
}

#[derive(Serialize)]
pub struct AppErrorResponse {
    pub error: String,
}

impl ResponseError for AppError {
    fn status_code(&self) -> StatusCode {
        match self.error_type {
            AppErrorType::InvalidRequest => StatusCode::BAD_REQUEST,
            AppErrorType::InvalidToken => StatusCode::UNAUTHORIZED,
            _ => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code()).json(AppErrorResponse {
            error: self.message(),
        })
    }
}

impl From<std::num::ParseIntError> for AppError {
    fn from(error: std::num::ParseIntError) -> AppError {
        AppError {
            message: None,
            cause: Some(error.to_string()),
            error_type: AppErrorType::DbError,
        }
    }
}

impl From<r2d2::Error> for AppError {
    fn from(error: r2d2::Error) -> AppError {
        AppError {
            message: None,
            cause: Some(error.to_string()),
            error_type: AppErrorType::PoolError,
        }
    }
}

impl From<diesel::result::Error> for AppError {
    fn from(error: diesel::result::Error) -> AppError {
        AppError {
            message: None,
            cause: Some(error.to_string()),
            error_type: AppErrorType::DbError,
        }
    }
}

impl From<actix_web::Error> for AppError {
    fn from(error: actix_web::Error) -> AppError {
        AppError {
            message: None,
            cause: Some(error.to_string()),
            error_type: AppErrorType::DbError,
        }
    }
}

impl From<actix_web::error::BlockingError<AppError>> for AppError {
    fn from(error: actix_web::error::BlockingError<AppError>) -> AppError {
        AppError {
            message: None,
            cause: Some(error.to_string()),
            error_type: AppErrorType::BlockingError,
        }
    }
}
