use serde::{Deserialize, Serialize};
use sqlx::Type;
use std::fmt::Display;
use utoipa::ToSchema;

/// 用户状态
#[derive(Debug, Clone, Copy, PartialEq, Default, Eq, Serialize, Deserialize, Type, ToSchema)]
#[sqlx(type_name = "user_status_enum", rename_all = "snake_case")] // 对应 Postgres 枚举类型名
#[serde(rename_all = "lowercase")]
pub enum UserStatus {
    #[default] // 标记默认变体
    #[serde(rename = "activate")]
    Activate,

    #[serde(rename = "deactivate")]
    Deactivate,

    #[serde(rename = "suspended")]
    Suspended,

    #[serde(rename = "locked")]
    Locked,

    #[serde(rename = "pending")]
    Pending,

    #[serde(rename = "deleted")]
    Deleted,
}

impl Display for UserStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            UserStatus::Activate => write!(f, "activate"),
            UserStatus::Deactivate => write!(f, "deactivate"),
            UserStatus::Suspended => write!(f, "suspended"),
            UserStatus::Locked => write!(f, "locked"),
            UserStatus::Pending => write!(f, "pending"),
            UserStatus::Deleted => write!(f, "deleted"),
        }
    }
}

impl UserStatus {
    #[allow(dead_code)]
    fn as_str(&self) -> &str {
        match self {
            UserStatus::Activate => "activate",
            UserStatus::Deactivate => "deactivate",
            UserStatus::Suspended => "suspended",
            UserStatus::Locked => "locked",
            UserStatus::Pending => "pending",
            UserStatus::Deleted => "deleted",
        }
    }
}
