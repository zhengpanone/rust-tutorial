// src/domain/services/value_objects/api_method.rs
use serde::{Deserialize, Serialize};
use thiserror::Error;
use utoipa::ToSchema;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "UPPERCASE")]
pub enum ApiMethod {
    GET = 1,
    POST = 2,
    PUT = 3,
    DELETE = 4,
    PATCH = 5,
    HEAD = 6,
    OPTIONS = 7,
}

impl Default for ApiMethod {
    fn default() -> Self {
        ApiMethod::GET
    }
}

impl TryFrom<i8> for ApiMethod {
    type Error = ApiMethodError;

    fn try_from(value: i8) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(ApiMethod::GET),
            2 => Ok(ApiMethod::POST),
            3 => Ok(ApiMethod::PUT),
            4 => Ok(ApiMethod::DELETE),
            5 => Ok(ApiMethod::PATCH),
            6 => Ok(ApiMethod::HEAD),
            7 => Ok(ApiMethod::OPTIONS),
            _ => Err(ApiMethodError::InvalidValue(value)),
        }
    }
}

impl From<ApiMethod> for i8 {
    fn from(method: ApiMethod) -> Self {
        match method {
            ApiMethod::GET => 1,
            ApiMethod::POST => 2,
            ApiMethod::PUT => 3,
            ApiMethod::DELETE => 4,
            ApiMethod::PATCH => 5,
            ApiMethod::HEAD => 6,
            ApiMethod::OPTIONS => 7,
        }
    }
}

#[derive(Debug, Error)]
pub enum ApiMethodError {
    #[error("无效的API方法值: {0}")]
    InvalidValue(i8),
}
