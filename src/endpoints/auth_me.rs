use crate::errors::AppError;
use crate::errors::AppErrorType;

use actix_identity::Identity;
use actix_web::{get, HttpResponse};
use serde::Serialize;

#[derive(Serialize)]
struct UserInfo {
    login: String,
}

#[get("/me")]
pub async fn index(id: Identity) -> Result<HttpResponse, AppError> {
    let identity = id.identity();
    // If the user is not identified, restrict access
    if !identity.is_some() {
        return Err(AppError {
            cause: None,
            message: Some("You're not allowed to access this resource".to_string()),
            error_type: AppErrorType::InvalidRequest,
        });
    }
    // Return the login for now, unwrap safely as it was checked just before
    Ok(HttpResponse::Ok().json(UserInfo {
        login: identity.unwrap(),
    }))
}
