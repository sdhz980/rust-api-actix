use core::fmt;

use actix_web::{ HttpResponse, ResponseError };
use serde_json::json;

#[derive(Debug)]
pub enum AppError {
    // Database errors
    DatabaseError(sqlx::Error),

    // User service specific errors
    UserNotFound(String),
    UserAlreadyExists(String),
    InvalidUserData(String),

    // Authentication errors
    Unauthorized(String),
    Forbidden(String),

    // Generic errors
    NotFound(String),
    BadRequest(String),
    InternalServerError(String),
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AppError::DatabaseError(err) => write!(f, "Database error: {}", err),
            AppError::UserNotFound(msg) => write!(f, "User not found: {}", msg),
            AppError::BadRequest(msg) => write!(f, "Bad request: {}", msg),
            AppError::InternalServerError(msg) => write!(f, "Internal server error: {}", msg),
            AppError::UserAlreadyExists(msg) => write!(f, "User already exist: {}", msg),
            AppError::InvalidUserData(msg) => write!(f, "Invalid user data: {}", msg),
            AppError::Unauthorized(msg) => write!(f, "Unauthorized: {}", msg),
            AppError::Forbidden(msg) => write!(f, "Forbidden: {}", msg),
            AppError::NotFound(msg) => write!(f, "Not Found: {}", msg),
        }
    }
}

impl ResponseError for AppError {
    fn error_response(&self) -> HttpResponse {
        let (status_code, error_code, message) = match self {
            AppError::DatabaseError(_err) => {
                // log(LogLevel::Error, format!("Database error: {}", err));
                (
                    actix_web::http::StatusCode::INTERNAL_SERVER_ERROR,
                    "DATABASE_ERROR",
                    "Internal server error occurred",
                )
            }
            AppError::UserNotFound(msg) => {
                // log(LogLevel::Error, format!("Users error: {}", msg));
                (actix_web::http::StatusCode::NOT_FOUND, "USER_NOT_FOUND", msg.as_str())
            }
            AppError::BadRequest(msg) => {
                // log(LogLevel::Error, format!("Bad request error: {}", msg));
                (actix_web::http::StatusCode::BAD_REQUEST, "BAD_REQUEST", msg.as_str())
            }
            AppError::InternalServerError(msg) => {
                // log(LogLevel::Error, format!("Internal server error: {}", msg));
                (
                    actix_web::http::StatusCode::INTERNAL_SERVER_ERROR,
                    "INTERNAL_SERVER_ERROR",
                    msg.as_str(),
                )
            }
            AppError::UserAlreadyExists(msg) =>
                (actix_web::http::StatusCode::CONFLICT, "USER_ALREADY_EXISTS", msg.as_str()),
            AppError::InvalidUserData(msg) =>
                (actix_web::http::StatusCode::BAD_REQUEST, "INVALID_USER_DATA", msg.as_str()),
            AppError::Unauthorized(msg) =>
                (actix_web::http::StatusCode::UNAUTHORIZED, "UNAUTHORIZED", msg.as_str()),
            AppError::Forbidden(msg) =>
                (actix_web::http::StatusCode::FORBIDDEN, "FORBIDDEN", msg.as_str()),
            AppError::NotFound(msg) =>
                (actix_web::http::StatusCode::NOT_FOUND, "NOT_FOUND", msg.as_str()),
        };

        HttpResponse::build(status_code).json(
            json!({
            "error": {
                "code": error_code,
                "message": message,
                "status": status_code.as_u16()
            }
        })
        )
    }

    fn status_code(&self) -> actix_web::http::StatusCode {
        match self {
            AppError::DatabaseError(_) => actix_web::http::StatusCode::INTERNAL_SERVER_ERROR,
            AppError::UserNotFound(_) => actix_web::http::StatusCode::NOT_FOUND,
            AppError::BadRequest(_) => actix_web::http::StatusCode::BAD_REQUEST,
            AppError::InternalServerError(_) => actix_web::http::StatusCode::INTERNAL_SERVER_ERROR,
            AppError::UserAlreadyExists(_) => actix_web::http::StatusCode::CONFLICT,
            AppError::InvalidUserData(_) => actix_web::http::StatusCode::BAD_REQUEST,
            AppError::Unauthorized(_) => actix_web::http::StatusCode::UNAUTHORIZED,
            AppError::Forbidden(_) => actix_web::http::StatusCode::FORBIDDEN,
            AppError::NotFound(_) => actix_web::http::StatusCode::NOT_FOUND,
        }
    }
}

// Convert dari SQLx errors
impl From<sqlx::Error> for AppError {
    fn from(err: sqlx::Error) -> Self {
        match err {
            sqlx::Error::RowNotFound => AppError::UserNotFound("Resource not found".to_string()),
            _ => AppError::DatabaseError(err),
        }
    }
}

pub type AppResult<T> = Result<T, AppError>;
