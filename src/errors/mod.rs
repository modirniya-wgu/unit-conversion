use actix_web::{HttpResponse, ResponseError};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ApiError {
    #[error("Internal server error: {0}")]
    InternalError(String),
    
    #[error("Bad request: {0}")]
    BadRequest(String),
    
    #[error("Not found: {0}")]
    NotFound(String),
    
    #[error("Unauthorized: {0}")]
    Unauthorized(String),
}

impl ResponseError for ApiError {
    fn error_response(&self) -> HttpResponse {
        match self {
            ApiError::InternalError(message) => {
                HttpResponse::InternalServerError().json(serde_json::json!({
                    "error": "internal_server_error",
                    "message": message
                }))
            },
            ApiError::BadRequest(message) => {
                HttpResponse::BadRequest().json(serde_json::json!({
                    "error": "bad_request",
                    "message": message
                }))
            },
            ApiError::NotFound(message) => {
                HttpResponse::NotFound().json(serde_json::json!({
                    "error": "not_found",
                    "message": message
                }))
            },
            ApiError::Unauthorized(message) => {
                HttpResponse::Unauthorized().json(serde_json::json!({
                    "error": "unauthorized",
                    "message": message
                }))
            }
        }
    }
} 