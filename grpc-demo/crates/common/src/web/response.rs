// src/web/response.rs
use axum::{
    Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde::Serialize;
use utoipa::ToSchema;

#[derive(Debug, Serialize, ToSchema)]
pub struct ApiResponse<T: Serialize> {
    pub code: i32,
    pub message: String,
    pub data: Option<T>,
    pub timestamp: i64,
}

impl<T: Serialize> ApiResponse<T> {
    pub fn success(data: T) -> Self {
        Self {
            code: 200,
            message: "success".to_string(),
            data: Some(data),
            timestamp: chrono::Utc::now().timestamp(),
        }
    }

    pub fn success_with_message(message: &str, data: T) -> Self {
        Self {
            code: 200,
            message: message.to_string(),
            data: Some(data),
            timestamp: chrono::Utc::now().timestamp(),
        }
    }

    pub fn created(data: T) -> Self {
        Self {
            code: 201,
            message: "created".to_string(),
            data: Some(data),
            timestamp: chrono::Utc::now().timestamp(),
        }
    }

    pub fn no_content() -> Self {
        Self {
            code: 204,
            message: "no content".to_string(),
            data: None,
            timestamp: chrono::Utc::now().timestamp(),
        }
    }

    pub fn error(code: i32, message: &str) -> Self {
        Self {
            code,
            message: message.to_string(),
            data: None,
            timestamp: chrono::Utc::now().timestamp(),
        }
    }

    pub fn bad_request(message: &str) -> Self {
        Self::error(400, message)
    }

    pub fn not_found(message: &str) -> Self {
        Self::error(404, message)
    }

    pub fn internal_error(message: &str) -> Self {
        Self::error(500, message)
    }
}

impl<T: Serialize> IntoResponse for ApiResponse<T> {
    fn into_response(self) -> Response {
        let status = match self.code {
            200 => StatusCode::OK,
            201 => StatusCode::CREATED,
            204 => StatusCode::NO_CONTENT,
            400 => StatusCode::BAD_REQUEST,
            404 => StatusCode::NOT_FOUND,
            500 => StatusCode::INTERNAL_SERVER_ERROR,
            _ => StatusCode::OK,
        };
        (status, Json(self)).into_response()
    }
}
