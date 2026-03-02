// src/domain/identity/entity/role.rs
// 角色领域实体
use crate::domain::shared::id::DomainId;
use chrono::{DateTime, Utc};
use common::enums::role::{RoleStatus, RoleType};
use common::error::AppError;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Newtype 模式
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RoleId(String);

impl DomainId for RoleId {
    fn from_string(value: String) -> Self {
        Self(value)
    }

    fn as_str(&self) -> &str {
        &self.0
    }
}

impl From<String> for RoleId {
    fn from(id: String) -> Self {
        Self(id)
    }
}

impl From<&str> for RoleId {
    fn from(value: &str) -> Self {
        Self(value.to_string())
    }
}

impl From<Uuid> for RoleId {
    fn from(uuid: Uuid) -> Self {
        Self(uuid.to_string())
    }
}

impl TryFrom<RoleId> for Uuid {
    type Error = AppError;

    fn try_from(value: RoleId) -> Result<Self, Self::Error> {
        Uuid::parse_str(&value.0).map_err(|_| AppError::InvalidId(value.0))
    }
}

impl std::fmt::Display for RoleId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SysRole {
    // 角色ID
    pub id: RoleId,
    // 角色名称
    pub role_name: String,

    pub role_code: String,

    pub role_type: RoleType, //角色类型：1-系统角色，2-业务角色，3-自定义角色'

    pub role_desc: Option<String>,

    // 角色状态
    pub status: RoleStatus,

    // 是否默认角色 是否用于自动分配、默认初始化角色，通常可删
    pub is_default: bool,

    // 是否保护角色 是否为系统核心角色，强保护，不允许删
    pub is_protected: bool,

    // 排序值
    pub order_num: i32,

    // 角色备注
    pub remark: String,

    // 创建时间
    pub created_at: DateTime<Utc>,

    // 创建人
    pub created_by: String,

    // 更新时间
    pub updated_at: DateTime<Utc>,

    // 更新人
    pub updated_by: String,

    // 是否删除
    pub is_deleted: Option<bool>,

    // 删除时间
    pub deleted_at: Option<DateTime<Utc>>,
}
