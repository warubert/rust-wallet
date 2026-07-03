
use axum::{http::StatusCode, response::IntoResponse, Json};
use serde::Serialize;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum AppError {
    #[error("Missing authorization header")]
    MissingAuthorization,
    #[error("Invalid Credentials")]
    InvalidCredentials,
    #[error("Asset not found")]
    AssetNotFound,
}

#[derive(Serialize)]
pub struct ErrorResponse {
    error: String,
}

impl IntoResponse for AppError {
    fn into_response(self) -> axum::response::Response {
        let error_response = ErrorResponse {
            error: self.to_string(),
        };

        let status_code = match self {
            Self::MissingAuthorization => StatusCode::BAD_REQUEST,
            Self::InvalidCredentials => StatusCode::UNAUTHORIZED,
            Self::AssetNotFound =>  StatusCode::NOT_FOUND,
        };

        (status_code, Json(error_response)).into_response()
    }
}