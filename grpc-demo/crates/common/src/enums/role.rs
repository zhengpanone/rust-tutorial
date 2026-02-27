use serde::{Deserialize, Serialize};
use sqlx::Type;
use thiserror::Error;
use utoipa::ToSchema;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "snake_case")]
pub enum RoleType {
    /// 系统角色
    System = 1,
    /// 内置角色
    BuiltIn = 2,
    /// 自定义角色
    Custom = 3,
}

#[derive(Debug, Clone, Serialize, Deserialize, Type, ToSchema)]
#[sqlx(type_name = "role_status_enum",rename_all = "lowercase")] // 对应 postgres 的类型名,如果枚举变体小写对应 db 字符串
pub enum RoleStatus {
    Active,
    Inactive,
    Banned,
}

impl Default for RoleType {
    fn default() -> Self {
        RoleType::Custom
    }
}

impl TryFrom<i8> for RoleType {
    type Error = RoleTypeError;

    fn try_from(value: i8) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(RoleType::System),
            2 => Ok(RoleType::BuiltIn),
            3 => Ok(RoleType::Custom),
            _ => Err(RoleTypeError::InvalidValue(value)),
        }
    }
}

impl From<RoleType> for i8 {
    fn from(role_type: RoleType) -> Self {
        match role_type {
            RoleType::System => 1,
            RoleType::BuiltIn => 2,
            RoleType::Custom => 3,
        }
    }
}

#[derive(Debug, Error)]
pub enum RoleTypeError {
    #[error("无效的角色类型值: {0}")]
    InvalidValue(i8),
}



#[allow(clippy::derivable_impls)]
impl Default for RoleStatus {
    fn default() -> Self {
        RoleStatus::Active
    }
}
