use serde::{Deserialize, Serialize};
use sqlx::Type;
use utoipa::ToSchema;

#[derive(Debug, Clone, Serialize, Deserialize, Type, ToSchema)]
#[sqlx(type_name = "role_status_enum")] // 对应 postgres 的类型名
#[sqlx(rename_all = "lowercase")] // 如果枚举变体小写对应 db 字符串
pub enum RoleStatus {
    Active,
    Inactive,
    Banned,
}

#[allow(clippy::derivable_impls)]
impl Default for RoleStatus {
    fn default() -> Self {
        RoleStatus::Active
    }
}
