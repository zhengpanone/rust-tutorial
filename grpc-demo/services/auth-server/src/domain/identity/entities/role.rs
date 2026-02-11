use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

use crate::enums::role_enums::RoleStatus;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct SysRole {
    // 角色ID
    pub id: String,
    // 角色名称
    pub role_name: String,

    pub role_code: String,
    pub role_type: String, //角色类型：1-系统角色，2-业务角色，3-自定义角色'

    pub role_desc: String,

    // 角色状态
    #[sqlx(default)]
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
    #[sqlx(default)]
    pub is_deleted: Option<bool>,
    // 删除时间
    #[sqlx(default)]
    pub deleted_at: Option<DateTime<Utc>>,
}
