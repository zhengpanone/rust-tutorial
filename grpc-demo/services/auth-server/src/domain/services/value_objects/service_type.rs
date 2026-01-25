// src/domain/services/value_objects/service_type.rs

use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub enum ServiceType {
    /// 业务服务
    Business = 1,
    /// 支撑服务
    Support = 2,
    /// 数据服务
    Data = 3,
}

impl From<i8> for ServiceType {
    fn from(value: i8) -> Self {
        match value {
            1 => ServiceType::Business,
            2 => ServiceType::Support,
            3 => ServiceType::Data,
            _ => ServiceType::Business,
        }
    }
}

impl From<ServiceType> for i8 {
    fn from(service_type: ServiceType) -> Self {
        match service_type {
            ServiceType::Business => 1,
            ServiceType::Support => 2,
            ServiceType::Data => 3,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub enum ServiceStatus {
    /// 启用
    Enabled = 1,
    /// 停用
    Disabled = 0,
}

impl From<i8> for ServiceStatus {
    fn from(value: i8) -> Self {
        match value {
            1 => ServiceStatus::Enabled,
            0 => ServiceStatus::Disabled,
            _ => ServiceStatus::Enabled,
        }
    }
}

impl From<ServiceStatus> for i8 {
    fn from(status: ServiceStatus) -> Self {
        match status {
            ServiceStatus::Enabled => 1,
            ServiceStatus::Disabled => 0,
        }
    }
}
