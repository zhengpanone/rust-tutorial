use crate::domain::identity::entity::user::UserId;
use chrono::{DateTime, Utc};
use common::enums::user::UserStatus;
use common::security::jwt::claim::{DeviceInfo, JwtClaim};
use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use tracing::error;
use utoipa::ToSchema;
use uuid::Uuid;
use validator::Validate;

/// 认证用户信息
#[derive(Debug, Clone, Serialize, Deserialize, Validate, ToSchema)]
pub struct AuthUser {
    /// 用户ID
    pub user_id: Uuid,
    /// 用户名
    pub username: String,

    /// 邮箱
    pub email: String,

    /// 显示名称
    pub display_name: String,

    /// 头像URL
    #[serde(skip_serializing_if = "Option::is_none")]
    pub avatar_url: Option<String>,

    /// 手机号
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone: Option<String>,

    /// 角色列表
    pub roles: HashSet<String>,

    /// 权限列表
    pub permissions: HashSet<String>,

    /// 令牌ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub token_id: Option<Uuid>,

    /// 会话ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub session_id: Option<Uuid>,

    /// 令牌类型
    #[serde(skip_serializing_if = "Option::is_none")]
    pub token_type: Option<String>,

    /// 是否已验证邮箱
    #[serde(default)]
    pub email_verified: bool,

    /// 是否已验证手机
    #[serde(default)]
    pub phone_verified: bool,

    /// 账户状态
    pub status: UserStatus,

    /// 令牌颁发时间
    #[serde(skip_serializing_if = "Option::is_none")]
    pub issued_at: Option<DateTime<Utc>>,

    /// 令牌过期时间
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expires_at: Option<DateTime<Utc>>,

    /// 是否首次登录
    #[serde(default)]
    pub is_first_login: bool,

    /// 登录时间
    #[serde(skip_serializing_if = "Option::is_none")]
    pub login_time: Option<DateTime<Utc>>,

    /// 最后活动时间
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_activity: Option<DateTime<Utc>>,

    /// 设备信息
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_info: Option<DeviceInfo>,

    /// 元数据
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<serde_json::Value>,
}

impl AuthUser {
    pub fn from_claims(claims: &JwtClaim) -> Self {
        let device_info = claims.session.device_info.as_ref().map(|info| DeviceInfo {
            device_id: info.device_id.clone(),
            device_type: info.device_type.clone(),
            user_agent: info.user_agent.clone(),
            ip_address: info.ip_address.clone(),
            location: info.location.clone(),
        });

        Self {
            user_id: Uuid::parse_str(&claims.sub).unwrap_or_else(|e| {
                error!("Failed to parse user_id from claims.sub: {}", e);
                Uuid::new_v4()
            }),
            username: claims.user.username.clone(),
            email: claims.user.email.clone(),
            display_name: claims.user.display_name.clone(),
            avatar_url: None,
            phone: None,
            roles: claims.user.roles.clone(),
            permissions: claims.user.permissions.clone(),
            token_id: Some(Uuid::parse_str(&claims.jti).unwrap_or_else(|e| {
                error!("Failed to parse user_id from claims.sub: {}", e);
                Uuid::new_v4()
            })),
            session_id: Some(claims.session.id),
            token_type: Some(claims.typ.to_string()),
            email_verified: claims.user.email_verified,
            phone_verified: claims.user.phone_verified,
            status: claims.user.status,
            issued_at: Some(DateTime::from_timestamp(claims.iat, 0).unwrap_or(Utc::now())),
            expires_at: Some(DateTime::from_timestamp(claims.exp, 0).unwrap_or(Utc::now())),
            is_first_login: claims.session.is_first_login,
            login_time: Some(claims.session.login_time),
            last_activity: Some(claims.session.last_activity),
            device_info,
            metadata: serde_json::to_value(claims.session.metadata.clone())
                .ok()
                .filter(|v| !v.is_null()),
        }
    }

    /// 检查用户是否已认证
    pub fn is_authenticated(&self) -> bool {
        self.user_id != Uuid::nil() && self.status == UserStatus::Activate
    }

    /// 检查用户是否具有指定角色
    pub fn has_role(&self, role: &str) -> bool {
        self.roles
            .iter()
            .any(|r| r == role || r.starts_with(&format!("{}:", role)))
    }

    /// 检查用户是否具有所有指定角色
    pub fn has_all_roles(&self, roles: &[&str]) -> bool {
        roles.iter().all(|r| self.has_role(r))
    }
    
    /// 检查用户是否具有任意指定角色
    pub fn has_any_role(&self, roles: &[&str]) -> bool {
        roles.iter().any(|r| self.has_role(r))
    }

    /// 检查用户是否具有指定权限
    pub fn has_permission(&self, permission: &str) -> bool {
        self.permissions.contains(permission)
    }

    /// 检查用户是否具有所有指定权限
    pub fn has_all_permissions(&self, permissions: &[&str]) -> bool {
        permissions.iter().all(|perm| self.has_permission(perm))
    }

    /// 检查用户是否具有任意指定权限
    pub fn has_any_permission(&self, permissions: &[&str]) -> bool {
        permissions.iter().any(|perm| self.has_permission(perm))
    }

    /// 检查用户是否是超级管理员
    pub fn is_super_admin(&self) -> bool {
        self.has_role("super_admin") || self.has_role("admin") && self.has_permission("*")
    }

    /// 检查用户是否是系统管理员
    pub fn is_system_admin(&self) -> bool {
        self.has_role("system_admin") || self.has_role("sys_admin")
    }

    /// 检查用户是否是安全管理员
    pub fn is_security_admin(&self) -> bool {
        self.has_role("security_admin") || self.has_role("sec_admin")
    }

    /// 检查用户是否是审计管理员
    pub fn is_audit_admin(&self) -> bool {
        self.has_role("audit_admin")
    }

    /// 获取用户角色列表
    pub fn get_roles(&self) -> Vec<String> {
        self.roles.iter().cloned().collect()
    }

    /// 获取用户权限列表
    pub fn get_permissions(&self) -> Vec<String> {
        self.permissions.iter().cloned().collect()
    }

    /// 添加角色
    pub fn add_role(&mut self, role: String) {
        self.roles.insert(role);
    }

    /// 添加权限
    pub fn add_permission(&mut self, permission: String) {
        self.permissions.insert(permission);
    }

    /// 移除角色
    pub fn remove_role(&mut self, role: &str) -> bool {
        self.roles.remove(role)
    }

    /// 移除权限
    pub fn remove_permission(&mut self, permission: &str) -> bool {
        self.permissions.remove(permission)
    }
}
