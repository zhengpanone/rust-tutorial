// src/domain/identity/entity/permission.rs
// 权限领域实体
use crate::domain::shared::id::Id;
use chrono::{DateTime, Utc};
use common::enums::permission::PermissionType;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PermissionTag;

/// 类型别名
pub type PermissionId = Id<PermissionTag>;

#[derive(Debug, Clone)]
pub struct Permission {
    pub id: PermissionId,
    /// 权限编码
    pub permission_code: String,
    /// 权限名称
    pub permission_name: String,
    pub description: Option<String>,
    pub permission_type: PermissionType,
    /// 资源
    pub resource: String,
    /// 操作
    pub action: String,
    pub parent_id: Option<PermissionId>,
    pub sort_order: i32,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
