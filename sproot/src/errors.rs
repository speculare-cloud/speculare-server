use actix_web::{error::ResponseError, http::StatusCode, HttpResponse};
use serde::Serialize;
use std::fmt;

#[derive(Debug, PartialEq)]
pub enum AppErrorType {
    DbError,
    NotFound,
    PoolError,
    InvalidRequest,
    InvalidToken,
    BlockingError,
    ServerError,
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
                cause: Some(cause),
                ..
            } => format!("{} : {}", message, cause),
            AppError {
                message: Some(message),
                ..
            } => message.clone(),
            AppError {
                cause: Some(cause), ..
            } => cause.clone(),
            AppError {
                error_type: AppErrorType::NotFound,
                ..
            } => "The requested ressource doesn't exists.".to_string(),
            AppError {
                error_type: AppErrorType::PoolError,
                ..
            } => "Cannot get a connection from the pool for the database".to_string(),
            AppError {
                error_type: AppErrorType::InvalidToken,
                ..
            } => "The token is invalid or has expired".to_string(),
            AppError {
                error_type: AppErrorType::InvalidRequest,
                ..
            } => "Invalid request".to_string(),
            AppError {
                error_type: AppErrorType::ServerError,
                ..
            } => "Server error".to_string(),
            _ => "An unexpected error has occurred".to_string(),
        }
    }
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        write!(f, "{}", self.message())
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
            AppErrorType::NotFound => StatusCode::NOT_FOUND,
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
        let error_type = match error {
            diesel::result::Error::NotFound => AppErrorType::NotFound,
            _ => AppErrorType::DbError,
        };
        AppError {
            message: None,
            cause: Some(error.to_string()),
            error_type,
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

impl From<actix_web::error::BlockingError> for AppError {
    fn from(error: actix_web::error::BlockingError) -> AppError {
        AppError {
            message: None,
            cause: Some(error.to_string()),
            error_type: AppErrorType::BlockingError,
        }
    }
}
