use actix_web::{http::StatusCode, HttpResponse, ResponseError};
use serde_json::json;
use sqlx::Error as SqlxError;
use thiserror::Error;

//define additional error types
#[derive(Debug, Error)]
pub enum AppError {
    #[error("Database error: {0}")]
    SqlxError(#[from] SqlxError),

    #[error("URL not found")]
    NotFound,

    #[error("Invalid URL")]
    InvalidUrl,
}

impl ResponseError for AppError {
    
    //conditionally retrieve error status code
    fn status_code(&self) -> StatusCode {
        match self {
            AppError::SqlxError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            AppError::NotFound => StatusCode::NOT_FOUND,
            AppError::InvalidUrl => StatusCode::BAD_REQUEST,
        }
    }

    //output error
    fn error_response(&self) -> HttpResponse {
        let status_code = self.status_code();
        let error_message = self.to_string();

        let message = json!({"error": error_message,});

        HttpResponse::build(status_code)
            .json(message)
    }
}