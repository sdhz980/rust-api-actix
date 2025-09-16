use actix_web::{ HttpResponse, ResponseError };
use serde_json::json;

#[derive(Debug, thiserror::Error)]
pub enum AppError {
    #[error("Database error : {0}")] Database(#[from] sqlx::Error),

    #[error("Validation error : {0}")] Validation(String),

    #[error("Authentication error : {0}")] Authentication(String),

    #[error("Authorization error : {0}")] Authorization(String),

    #[error("Not found: {0}")] NotFound(String),

    #[error("Internal server error")]
    InternalServer,
}

impl ResponseError for AppError {
    fn error_response(&self) -> HttpResponse {
        match self {
            AppError::Database(_) =>
                HttpResponse::InternalServerError().json(
                    json!({"error" : "Database error occured"})
                ),
            AppError::Validation(msg) => HttpResponse::BadRequest().json(json!({"error" : msg})),
            AppError::Authentication(msg) =>
                HttpResponse::InternalServerError().json(json!({"error" : msg})),
            AppError::Authorization(msg) =>
                HttpResponse::InternalServerError().json(json!({"error" : msg})),
            AppError::NotFound(msg) =>
                HttpResponse::InternalServerError().json(json!({"error" : msg})),
            AppError::InternalServer =>
                HttpResponse::InternalServerError().json(
                    json!({"error" : "Internal server error"})
                ),
        }
    }
}

pub type AppResult<T> = Result<T, AppError>;
