use chrono::{DateTime, Utc};
use common::enums::permission::PermissionType;
use common::error::{AppError, AppResult};
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PermissionId(String);

impl PermissionId {
    pub fn new() -> Self {
        Self(Uuid::new_v4().to_string())
    }

    pub fn parse(id: &str) -> AppResult<Self> {
        Uuid::parse_str(id)
            .map_err(|_| AppError::InvalidId(id.to_string()))
            .map(|_| Self(id.to_string()))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

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
