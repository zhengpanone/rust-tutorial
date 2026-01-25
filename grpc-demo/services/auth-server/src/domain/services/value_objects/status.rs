// src/domain/services/value_objects/status.rs
use serde::{Deserialize, Serialize};
use thiserror::Error;
use utoipa::ToSchema;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "lowercase")]
pub enum ServiceStatus {
    /// 启用
    Enabled = 1,
    /// 停用
    Disabled = 0,
    /// 维护中
    Maintenance = 2,
}

impl Default for ServiceStatus {
    fn default() -> Self {
        ServiceStatus::Enabled
    }
}

impl TryFrom<i8> for ServiceStatus {
    type Error = StatusError;

    fn try_from(value: i8) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(ServiceStatus::Enabled),
            0 => Ok(ServiceStatus::Disabled),
            2 => Ok(ServiceStatus::Maintenance),
            _ => Err(StatusError::InvalidServiceStatus(value)),
        }
    }
}

impl From<ServiceStatus> for i8 {
    fn from(status: ServiceStatus) -> Self {
        match status {
            ServiceStatus::Enabled => 1,
            ServiceStatus::Disabled => 0,
            ServiceStatus::Maintenance => 2,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "lowercase")]
pub enum ApiStatus {
    /// 发布
    Published = 1,
    /// 草稿
    Draft = 0,
    /// 已废弃
    Deprecated = 2,
}

impl Default for ApiStatus {
    fn default() -> Self {
        ApiStatus::Draft
    }
}

impl TryFrom<i8> for ApiStatus {
    type Error = StatusError;

    fn try_from(value: i8) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(ApiStatus::Published),
            0 => Ok(ApiStatus::Draft),
            2 => Ok(ApiStatus::Deprecated),
            _ => Err(StatusError::InvalidApiStatus(value)),
        }
    }
}

impl From<ApiStatus> for i8 {
    fn from(status: ApiStatus) -> Self {
        match status {
            ApiStatus::Published => 1,
            ApiStatus::Draft => 0,
            ApiStatus::Deprecated => 2,
        }
    }
}

#[derive(Debug, Error)]
pub enum StatusError {
    #[error("无效的服务状态值: {0}")]
    InvalidServiceStatus(i8),

    #[error("无效的API状态值: {0}")]
    InvalidApiStatus(i8),
}
