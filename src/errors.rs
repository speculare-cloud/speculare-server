use actix_web::{error::ResponseError, http::StatusCode, HttpResponse};
use serde::Serialize;
use std::fmt;

#[derive(Debug)]
pub enum AppErrorType {
    RusotoError,
    DbError,
    PoolError,
    InvalidCrendetials,
    KeyAlreadyExists,
    InvalidToken,
    InvalidRequest,
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
                error_type: AppErrorType::KeyAlreadyExists,
                ..
            } => "The requested item is already present".to_string(),
            AppError {
                message: None,
                error_type: AppErrorType::RusotoError,
                ..
            } => "There was an error communicating with S3".to_string(),
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
                error_type: AppErrorType::InvalidCrendetials,
                ..
            } => "Your email was entered incorrectly, or your password was incorrect".to_string(),
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
            AppErrorType::KeyAlreadyExists => StatusCode::CONFLICT,
            AppErrorType::InvalidToken => StatusCode::UNAUTHORIZED,
            AppErrorType::InvalidCrendetials => StatusCode::FORBIDDEN,
            AppErrorType::InvalidRequest => StatusCode::BAD_REQUEST,
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

impl<E> From<actix_threadpool::BlockingError<E>> for AppError
where
    E: std::fmt::Debug,
    E: Into<AppError>,
{
    fn from(error: actix_threadpool::BlockingError<E>) -> AppError {
        match error {
            actix_threadpool::BlockingError::Error(e) => e.into(),
            actix_threadpool::BlockingError::Canceled => AppError {
                message: None,
                cause: None,
                error_type: AppErrorType::DbError,
            },
        }
    }
}
