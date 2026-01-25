// 服务表
// src/models/service.rs
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use utoipa::{IntoParams, ToSchema};

use crate::enums::service_enums::{ServiceStatus, ServiceType};

#[derive(Debug, Clone, Serialize, Deserialize, FromRow, ToSchema)]
pub struct Service {
    /// 服务ID
    pub id: String,
    /// 服务编码
    pub service_code: String,
    /// 服务名称
    pub service_name: String,
    /// 服务类型
    #[sqlx(try_from = "i8")]
    pub service_type: ServiceType,
    /// 服务描述
    pub service_desc: Option<String>,
    /// 负责团队
    pub owner_team: Option<String>,
    /// 服务基础URL
    pub base_url: Option<String>,
    /// 状态
    #[sqlx(try_from = "i8")]
    pub status: ServiceStatus,
    /// 健康检查端点
    pub health_endpoint: Option<String>,
    /// 是否内部服务
    pub is_internal: bool,
    /// 创建时间
    pub created_at: DateTime<Utc>,
    /// 更新时间
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize, IntoParams)]
pub struct ServiceQuery {
    pub service_code: Option<String>,
    pub service_name: Option<String>,
    pub service_type: Option<ServiceType>,
    pub status: Option<ServiceStatus>,
    pub is_internal: Option<bool>,
    pub page: Option<i64>,
    pub page_size: Option<i64>,
}
