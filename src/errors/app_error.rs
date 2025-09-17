use actix_web::{ HttpResponse, ResponseError };
use serde_json::json;

#[derive(Debug, thiserror::Error)]
pub enum AppError {
    #[error("Database error : {0}")] Database(#[from] sqlx::Error),

    #[error("Validation error : {0}")] Validation(String),

    #[error("Authentication error : {0}")] Authentication(String),

    #[error("Authorization error : {0}")] Authorization(String),

    #[error("Not found")] NotFound(),

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
            AppError::NotFound() =>
                HttpResponse::NotFound().json(
                    json!({
                    "error": {
                        "code": "NOT_FOUND",
                        "message": "The requested resource was not found",
                        "status": 404
                    }
                    })
                ),
            AppError::InternalServer =>
                HttpResponse::InternalServerError().json(
                    json!({
                    "error": {
                        "code": "INTERNAL_SERVER_ERROR",
                        "message": "An internal server error occurred",
                        "status": 500
                        }
                    })
                ),
        }
    }
}

pub type AppResult<T> = Result<T, AppError>;
