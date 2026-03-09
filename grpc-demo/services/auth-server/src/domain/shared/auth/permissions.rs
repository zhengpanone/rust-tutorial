use crate::app::middleware::http::auth::extractor::AuthUser;
use anyhow::bail;
use common::enums::user::UserStatus;
use common::error::{AppError, AppResult};
use tracing::{debug, info, warn};

/// 检查用户是否拥有指定角色
///
/// # 参数
/// - user: 用户信息
///
/// # 返回
/// - 如果用户拥有指定角色，则返回Ok(())，否则返回Err(AppError::Forbidden)
///
/// # 示例
/// ```rust
/// use crate::app::middleware::http::auth::extractor::AuthUser;
///
/// let user = AuthUser::default();
/// // 检查用户是否拥有管理员角色
/// require_role(&user, "admin")?;
/// ```
pub fn require_role(user: &AuthUser, role: &str) -> AppResult<()> {
    debug!("Checking role '{}' for user '{}'", role, user.username);
    // 检查用户是否已认证
    if !user.is_authenticated() {
        warn!("Unauthenticated user attempted to access role-protected resource");
        AppError::Authentication("Unauthenticated user".to_string());
    }

    // 检查用户状态
    if user.status != UserStatus::Activate {
        warn!(
            "Inactive user '{}' attempted to access role-protected resource",
            user.username
        );
        AppError::Authentication("User is not active".to_string());
    }

    // 检查角色
    if !user.has_role(role) {
        warn!(
            "User '{}' does not have required role '{}'",
            user.username, role
        );

    }
    info!("User '{}' has required role '{}'", user.username, role);
    Ok(())
}
