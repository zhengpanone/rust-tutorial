use serde::{Deserialize, Serialize};
use validator::Validate;

/// 安全配置
#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct SecurityConfig {
    /// JWT密钥
    #[validate(length(min = 32, message = "JWT secret must be at least 32 characters"))]
    pub jwt_secret: String,

    /// JWT过期时间（分钟）
    #[validate(range(min = 1, max = 1440))]
    pub jwt_expiry_minutes: u64,

    /// 刷新令牌过期时间（天）
    #[validate(range(min = 1, max = 365))]
    pub refresh_token_expiry_days: u64,

    /// 密码哈希成本
    #[validate(range(min = 8, max = 31))]
    pub password_hash_cost: u32,

    /// 最大登录尝试次数
    #[validate(range(min = 1, max = 20))]
    pub max_login_attempts: u32,

    /// 账户锁定时间（分钟）
    pub lockout_duration_minutes: u32,

    /// 是否需要邮箱验证
    pub require_email_verification: bool,

    /// 是否需要手机验证
    pub require_phone_verification: bool,

    /// 是否启用MFA
    pub enable_mfa: bool,

    /// 是否启用验证码
    pub enable_captcha: bool,

    /// 会话超时时间（分钟）
    pub session_timeout_minutes: u32,

    /// 是否启用CSRF保护
    pub enable_csrf_protection: bool,

    /// 是否启用XSS保护
    pub enable_xss_protection: bool,

    /// 是否启用CSP
    pub enable_content_security_policy: bool,

    /// 是否启用HSTS
    pub enable_hsts: bool,

    /// 安全头部
    pub security_headers: SecurityHeadersConfig,

    /// 令牌吊销列表启用
    pub enable_revocation_list: bool,

    /// 吊销列表清理间隔（小时）
    pub revocation_list_cleanup_hours: u32,

    /// 密码策略
    pub password_policy: PasswordPolicyConfig,
}

/// 安全头部配置
#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct SecurityHeadersConfig {
    /// 是否启用X-Frame-Options
    pub enable_x_frame_options: bool,

    /// 是否启用X-Content-Type-Options
    pub enable_x_content_type_options: bool,

    /// 是否启用Referrer-Policy
    pub enable_referrer_policy: bool,

    /// 是否启用Permissions-Policy
    pub enable_permissions_policy: bool,

    /// 是否启用Feature-Policy
    pub enable_feature_policy: bool,

    /// CSP 策略
    pub content_security_policy: String,

    /// 缓存控制
    pub cache_control: String,
}

/// 密码策略配置
#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct PasswordPolicyConfig {
    /// 最小密码长度
    #[validate(range(min = 8, max = 128))]
    pub min_length: usize,

    /// 是否需要大写字母
    pub require_uppercase: bool,

    /// 是否需要小写字母
    pub require_lowercase: bool,

    /// 是否需要数字
    pub require_digit: bool,

    /// 是否需要特殊字符
    pub require_special_char: bool,

    /// 密码历史记录数量
    pub password_history_count: u32,

    /// 密码最小使用天数
    pub min_password_age_days: u32,

    /// 密码最大使用天数
    pub max_password_age_days: u32,

    /// 禁止的密码列表
    pub forbidden_passwords: Vec<String>,
}

impl Default for SecurityConfig {
    fn default() -> Self {
        Self {
            jwt_secret: "your-super-secret-jwt-key-change-in-production".to_string(),
            jwt_expiry_minutes: 60,
            refresh_token_expiry_days: 7,
            password_hash_cost: 12,
            max_login_attempts: 5,
            lockout_duration_minutes: 15,
            require_email_verification: true,
            require_phone_verification: false,
            enable_mfa: false,
            enable_captcha: false,
            session_timeout_minutes: 30,
            enable_csrf_protection: true,
            enable_xss_protection: true,
            enable_content_security_policy: true,
            enable_hsts: true,
            security_headers: SecurityHeadersConfig::default(),
            enable_revocation_list: true,
            revocation_list_cleanup_hours: 24,
            password_policy: PasswordPolicyConfig::default(),
        }
    }
}

impl Default for SecurityHeadersConfig {
    fn default() -> Self {
        Self {
            enable_x_frame_options: true,
            enable_x_content_type_options: true,
            enable_referrer_policy: true,
            enable_permissions_policy: true,
            enable_feature_policy: true,
            content_security_policy: "default-src 'self'".to_string(),
            cache_control: "no-store, no-cache, must-revalidate".to_string(),
        }
    }
}

impl Default for PasswordPolicyConfig {
    fn default() -> Self {
        Self {
            min_length: 8,
            require_uppercase: true,
            require_lowercase: true,
            require_digit: true,
            require_special_char: true,
            password_history_count: 5,
            min_password_age_days: 1,
            max_password_age_days: 90,
            forbidden_passwords: vec![
                "password".to_string(),
                "123456".to_string(),
                "qwerty".to_string(),
                "admin".to_string(),
                "welcome".to_string(),
            ],
        }
    }
}

