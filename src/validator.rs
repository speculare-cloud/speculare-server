use crate::errors::{AppError, AppErrorType};

use actix_web::{dev::ServiceRequest, Error};
use actix_web_httpauth::extractors::{
    bearer::{BearerAuth, Config},
    AuthenticationError,
};

// Might become a true OAuth2.0 API in the future
fn validate_token(token: &str) -> Result<bool, AppError> {
    let rtoken = crate::TOKEN.as_ref();
    match rtoken {
        Ok(rtok) => Ok(token == *rtok),
        Err(_) => Err(AppError {
            message: None,
            cause: None,
            error_type: AppErrorType::InvalidToken,
        }),
    }
}

pub async fn validator(
    req: ServiceRequest,
    credentials: BearerAuth,
) -> Result<ServiceRequest, Error> {
    match validate_token(credentials.token()) {
        Ok(res) => {
            if res {
                Ok(req)
            } else {
                let config = req
                    .app_data::<Config>()
                    .cloned()
                    .unwrap_or_else(Default::default);
                Err(AuthenticationError::from(config).into())
            }
        }
        Err(res) => Err(res.into()),
    }
}
