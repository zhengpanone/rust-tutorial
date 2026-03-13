use std::collections::HashSet;
use thiserror::Error;

/// 认证错误
#[derive(Debug, Error)]
pub enum AuthError {
    /// 未认证
    #[error("User is not authenticated")]
    Unauthenticated,

    /// 认证过期
    #[error("Authentication token has expired")]
    TokenExpired,

    /// 无效的认证令牌
    #[error("Invalid authentication token")]
    InvalidToken,

    /// 用户不存在
    #[error("User does not exist")]
    UserNotFound,

    /// 用户被禁用
    #[error("User account is disabled")]
    UserDisabled,

    /// 用户被锁定
    #[error("User account is locked")]
    UserLocked,

    /// 用户不活跃
    #[error("User account is not active")]
    UserInactive,

    /// 无效的凭据
    #[error("Invalid credentials")]
    InvalidCredentials,

    /// 认证失败
    #[error("Authentication failed: {0}")]
    Failed(String),

    /// 权限不足
    #[error("Insufficient permissions")]
    InsufficientPermissions,

    /// 访问被拒绝
    #[error("Access denied")]
    AccessDenied,

    /// 会话过期
    #[error("Session has expired")]
    SessionExpired,

    /// 无效的会话
    #[error("Invalid session")]
    InvalidSession,

    /// 认证服务不可用
    #[error("Authentication service is unavailable")]
    ServiceUnavailable,

    /// 认证配置错误
    #[error("Authentication configuration error: {0}")]
    ConfigurationError(String),
}

/// 权限错误
#[derive(Debug, Error)]
pub enum PermissionError {
    /// 角色不足
    #[error("Insufficient role. Required: {required}, Actual: {actual:?}")]
    InsufficientRole {
        required: String,
        actual: HashSet<String>,
    },

    /// 缺少角色
    #[error(
        "Missing required roles. Required: {required:?}, Missing: {missing:?}, Actual: {actual:?}"
    )]
    MissingRoles {
        required: HashSet<String>,
        missing: Vec<String>,
        actual: HashSet<String>,
    },

    /// 没有匹配的角色
    #[error("No matching role found. Required any of: {required:?}, Actual: {actual:?}")]
    NoMatchingRole {
        required: HashSet<String>,
        actual: HashSet<String>,
    },

    /// 权限不足
    #[error("Insufficient permission. Required: {required}, Actual: {actual:?}")]
    InsufficientPermission {
        required: String,
        actual: HashSet<String>,
    },

    /// 缺少权限
    #[error(
        "Missing required permissions. Required: {required:?}, Missing: {missing:?}, Actual: {actual:?}"
    )]
    MissingPermissions {
        required: HashSet<String>,
        missing: Vec<String>,
        actual: HashSet<String>,
    },

    /// 没有匹配的权限
    #[error("No matching permission found. Required any of: {required:?}, Actual: {actual:?}")]
    NoMatchingPermission {
        required: HashSet<String>,
        actual: HashSet<String>,
    },

    /// 角色级别不足
    #[error(
        "Insufficient role level. Required: {required}, Actual: {actual}, Hierarchy: {hierarchy:?}"
    )]
    InsufficientRoleLevel {
        required: String,
        actual: String,
        hierarchy: Vec<String>,
    },

    /// 没有层次结构中的角色
    #[error(
        "No role from hierarchy. Required: {required}, Hierarchy: {hierarchy:?}, Actual: {actual:?}"
    )]
    NoHierarchyRole {
        required: String,
        hierarchy: Vec<String>,
        actual: HashSet<String>,
    },

    /// 无效的角色层次结构
    #[error("Invalid role hierarchy. Required: {required}, Hierarchy: {hierarchy:?}")]
    InvalidRoleHierarchy {
        required: String,
        hierarchy: Vec<String>,
    },

    /// 资源未找到
    #[error("Resource not found: {resource}")]
    ResourceNotFound { resource: String },

    /// 操作不允许
    #[error("Operation not allowed: {operation}")]
    OperationNotAllowed { operation: String },

    /// 租户不匹配
    #[error("Tenant mismatch. Required: {required}, Actual: {actual}")]
    TenantMismatch { required: String, actual: String },

    /// 权限检查失败
    #[error("Permission check failed: {reason}")]
    CheckFailed { reason: String },
}

/// 认证结果类型别名
pub type AuthResult<T> = Result<T, AuthError>;

/// 权限结果类型别名
pub type PermissionResult<T> = Result<T, PermissionError>;
