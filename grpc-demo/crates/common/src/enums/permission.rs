use serde::{Deserialize, Serialize};
use thiserror::Error;
use utoipa::ToSchema;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "snake_case")]
pub enum PermissionType {
    /// 菜单权限
    Menu = 1,
    /// 操作权限
    Action = 2,
    /// 数据权限
    Data = 3,
    /// API权限
    Api = 4,
}

impl Default for PermissionType {
    fn default() -> Self {
        PermissionType::Action
    }
}

impl TryFrom<i8> for PermissionType {
    type Error = PermissionTypeError;

    fn try_from(value: i8) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(PermissionType::Menu),
            2 => Ok(PermissionType::Action),
            3 => Ok(PermissionType::Data),
            4 => Ok(PermissionType::Api),
            _ => Err(PermissionTypeError::InvalidValue(value)),
        }
    }
}

impl From<PermissionType> for i8 {
    fn from(permission_type: PermissionType) -> Self {
        match permission_type {
            PermissionType::Menu => 1,
            PermissionType::Action => 2,
            PermissionType::Data => 3,
            PermissionType::Api => 4,
        }
    }
}

#[derive(Debug, Error)]
pub enum PermissionTypeError {
    #[error("无效的权限类型值: {0}")]
    InvalidValue(i8),
}
