use actix_web::{http::StatusCode, HttpResponse, ResponseError};
use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("Authentication error: {0}")]
    Auth(String),

    #[error("Bad request: {0}")]
    BadRequest(String),

    #[error("Not found: {0}")]
    NotFound(String),

    #[error("Rate limit exceeded")]
    RateLimitExceeded,

    #[error("Internal server error: {0}")]
    InternalServerError(String),

    #[error("Solana error: {0}")]
    SolanaError(String),

    #[error("Serialization error: {0}")]
    SerializationError(String),
}

#[derive(Serialize, Deserialize)]
pub struct ErrorResponse {
    pub status: String,
    pub message: String,
}

impl ResponseError for AppError {
    fn status_code(&self) -> StatusCode {
        match self {
            AppError::Auth(_) => StatusCode::UNAUTHORIZED,
            AppError::BadRequest(_) => StatusCode::BAD_REQUEST,
            AppError::NotFound(_) => StatusCode::NOT_FOUND,
            AppError::RateLimitExceeded => StatusCode::TOO_MANY_REQUESTS,
            AppError::InternalServerError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            AppError::SolanaError(_) => StatusCode::BAD_GATEWAY,
            AppError::SerializationError(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    fn error_response(&self) -> HttpResponse {
        let status = self.status_code();
        let error_response = ErrorResponse {
            status: status.to_string(),
            message: self.to_string(),
        };
        HttpResponse::build(status).json(error_response)
    }
}

// Utility functions for creating errors
impl AppError {
    pub fn bad_request<T: ToString>(msg: T) -> Self {
        AppError::BadRequest(msg.to_string())
    }

    pub fn not_found<T: ToString>(msg: T) -> Self {
        AppError::NotFound(msg.to_string())
    }

    pub fn auth<T: ToString>(msg: T) -> Self {
        AppError::Auth(msg.to_string())
    }

    pub fn internal<T: ToString>(msg: T) -> Self {
        AppError::InternalServerError(msg.to_string())
    }

    pub fn solana<T: ToString>(msg: T) -> Self {
        AppError::SolanaError(msg.to_string())
    }
}

// Implement From for common errors
impl From<std::io::Error> for AppError {
    fn from(error: std::io::Error) -> Self {
        AppError::InternalServerError(error.to_string())
    }
}

impl From<serde_json::Error> for AppError {
    fn from(error: serde_json::Error) -> Self {
        AppError::SerializationError(error.to_string())
    }
}

impl From<anyhow::Error> for AppError {
    fn from(error: anyhow::Error) -> Self {
        AppError::InternalServerError(error.to_string())
    }
}

// Result type alias for easy error handling throughout the application
pub type AppResult<T> = Result<T, AppError>; 