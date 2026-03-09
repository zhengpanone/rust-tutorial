// src/application/init/mod.rs
//! 应用初始化服务
//! 负责在应用启动时执行初始化任务

use crate::domain::identity::entity::permission::{Permission, PermissionId};

mod account_init;

#[derive(Debug, Clone)]
pub struct InitConfig {
    pub enabled: bool,
    /// 超级管理员配置
    pub super_admin: SuperAdminConfig,
    /// 三权账号配置
    pub three_auth: ThreePrivilegeConfig,
}
/// 超级管理员配置
#[derive(Debug, Clone)]
pub struct SuperAdminConfig {
    /// 邮箱
    pub email: String,
    /// 用户名
    pub username: String,
    /// 密码
    pub password: String,
    /// 姓
    pub first_name: String,
    /// 名
    pub last_name: String,
    /// 描述
    pub description: String,
    /// 是否启用
    pub enabled: bool,
    /// 是否强制重置密码
    pub force_password_reset: bool,
}

impl Default for SuperAdminConfig {
    fn default() -> Self {
        Self {
            email: "super.admin@example.com".to_string(),
            username: "superadmin".to_string(),
            password: "Super@123456".to_string(),
            first_name: "Super".to_string(),
            last_name: "Admin".to_string(),
            description: "".to_string(),
            enabled: true,
            force_password_reset: true,
        }
    }
}

/// 三权账号配置
#[derive(Debug, Clone)]
pub struct ThreePrivilegeConfig {
    /// 系统管理员配置
    pub sys_admin: AccountConfig,
    /// 安全管理员配置
    pub sec_admin: AccountConfig,
    /// 审计管理员配置
    pub audit_admin: AccountConfig,
    /// 是否启用
    pub enabled: bool,
}

impl Default for ThreePrivilegeConfig {
    fn default() -> Self {
        Self {
            sys_admin: AccountConfig {
                email: "system.admin@example.com".to_string(),
                username: "sysadmin".to_string(),
                password: "Sys@123456".to_string(),
                first_name: "System".to_string(),
                last_name: "Admin".to_string(),
                enabled: true,
                description: "系统管理员，负责系统日常运维管理".to_string(),
            },
            sec_admin: AccountConfig {
                email: "security.admin@example.com".to_string(),
                username: "secadmin".to_string(),
                password: "Sec@123456".to_string(),
                first_name: "Security".to_string(),
                last_name: "Admin".to_string(),
                enabled: true,
                description: "安全管理员，负责系统安全策略配置".to_string(),
            },
            audit_admin: AccountConfig {
                email: "audit.admin@example.com".to_string(),
                username: "auditadmin".to_string(),
                password: "Audit@123456".to_string(),
                first_name: "Audit".to_string(),
                last_name: "Admin".to_string(),
                enabled: true,
                description: "审计管理员，负责系统操作审计".to_string(),
            },
            enabled: true,
        }
    }
}

/// 账号配置
#[derive(Debug, Clone)]
pub struct AccountConfig {
    /// 邮箱
    pub email: String,
    /// 用户名
    pub username: String,
    /// 密码
    pub password: String,
    /// 姓
    pub first_name: String,
    /// 名
    pub last_name: String,
    /// 是否启用
    pub enabled: bool,
    /// 描述
    pub description: String,
}

/// 默认角色配置
#[derive(Debug, Clone)]
pub struct DefaultRoleConfig {
    /// 角色名称
    pub name: String,
    /// 角色代码
    pub code: String,
    /// 角色描述
    pub description: String,
    /// 是否启用
    pub enabled: bool,
    /// 是否为系统内置角色
    pub is_system: bool,
    /// 权限列表
    pub permissions: Vec<String>,
}

pub fn init_permission() {
    let super_permission = Permission {
        id: PermissionId::parse("1").unwrap(),
        permission_code: "*".to_string(),
        permission_name: "所有权限".to_string(),
        description: Some("所有权限".to_string()),
        permission_type: Default::default(),
        resource: "*".to_string(),
        action: "*".to_string(),
        parent_id: None,
        sort_order: 0,
        created_at: Default::default(),
        updated_at: Default::default(),
    };
    let user_permission = Permission {
        id: PermissionId::parse("2").unwrap(),
        permission_code: "SYS_ADMIN".to_string(),
        permission_name: "用户权限".to_string(),
        description: Some("用户权限".to_string()),
        permission_type: Default::default(),
        resource: "user".to_string(),
        action: "*".to_string(),
        parent_id: Some(PermissionId::parse("1").unwrap()),
        sort_order: 1,
        created_at: Default::default(),
        updated_at: Default::default(),
    };
}

pub fn init_role() {
    let role_config = DefaultRoleConfig {
        name: "系统管理员".to_string(),
        code: "sysadmin".to_string(),
        description: "系统管理员，负责系统日常运维管理".to_string(),
        enabled: true,
        is_system: true,
        permissions: vec!["*:*:*".to_string()],
    };
}
