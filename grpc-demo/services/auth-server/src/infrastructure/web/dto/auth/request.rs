// src/infrastructure/web/dto/auth/requests.rs

use once_cell::sync::Lazy;
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;
use utoipa::ToSchema;
use validator::{Validate, ValidationError};
use validator_ext::must_be_true_validator;

#[derive(Debug, Clone, Serialize, Deserialize, Validate, ToSchema)]
pub struct LoginRequest {
    /// 用户标识符(用户名、邮箱或手机号)
    #[validate(
        length(min = 1, max = 100, message = "用户名长度必须在1-100个字符之间"),
        custom(function = "validate_identifier")
    )]
    pub identifier: String,
    /// 密码
    #[validate(
        length(min = 6, max = 128, message = "密码长度必须在6-128个字符之间"),
        custom(function = "validate_password_format")
    )]
    pub password: String,

    /// 设备ID
    #[validate(length(min = 1, max = 100, message = "设备ID长度必须在1-100个字符之间"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_id: Option<String>,

    /// 设备类型
    #[validate(length(min = 1, max = 50, message = "设备类型长度必须在1-50个字符之间"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_type: Option<String>,
    /// 用户代理
    #[validate(length(min = 1, max = 500, message = "设备类型长度必须在1-500个字符之间"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_agent: Option<String>,

    /// IP地址
    #[validate(length(min = 1, max = 45, message = "设备类型长度必须在1-50个字符之间"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip_address: Option<String>,

    //// 地理位置
    #[validate(length(min = 1, max = 100, message = "设备类型长度必须在1-50个字符之间"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,

    /// 验证码
    #[validate(length(min = 1, max = 10, message = "设备类型长度必须在1-10个字符之间"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub captcha_code: Option<String>,

    /// 验证码ID
    #[validate(length(min = 1, max = 100, message = "设备类型长度必须在1-100个字符之间"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub captcha_id: Option<String>,

    /// 记住我
    #[serde(default)]
    pub remember_me: bool,

    /// 多因素认证代码
    #[validate(length(min = 1, max = 10, message = "MFA长度必须在1-10个字符之间"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mfa_code: Option<String>,
    /// 登录方式
    #[serde(default = "default_login_method")]
    pub login_method: LoginMethod,
}

/// 注册请求
#[must_be_true_validator]
#[derive(Debug, Clone, Serialize, Deserialize, Validate, ToSchema)]
pub struct RegisterRequest {
    /// 用户名
    #[validate(
        length(min = 3, max = 50, message = "用户名长度必须在3-50个字符之间"),
        custom(function = "validate_username")
    )]
    pub username: String,
    /// 邮箱
    #[validate(
        length(min = 1, max = 100, message = "邮箱长度必须在1-100个字符之间"),
        email(message = "邮箱格式不正确")
    )]
    pub email: String,
    /// 手机号
    #[validate(
        length(min = 1, max = 20, message = "手机号长度不能超过20个字符"),
        custom(function = "validate_phone")
    )]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone: Option<String>,

    #[validate(
        length(min = 6, max = 128, message = "密码长度必须在6-128个字符之间"),
        custom(function = "validate_password_strength")
    )]
    pub password: String,

    /// 确认密码
    #[validate(must_match(other = "password", message = "两次输入的密码不一致"))]
    pub confirm_password: String,

    #[validate(length(min = 1, max = 100, message = "显示名称长度不能超过100个字符"))]
    pub display_name: String,
    /// 头像URL
    #[validate(
        length(max = 500, message = "头像URL长度不能超过500个字符"),
        url(message = "头像URL格式不正确")
    )]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub avatar_url: Option<String>,

    /// 注册来源
    #[validate(length(max = 50, message = "来源长度不能超过50个字符"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<String>,

    /// 邀请码
    #[validate(length(max = 50, message = "邀请码长度不能超过50个字符"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invitation_code: Option<String>,

    /// 验证码
    #[validate(length(max = 10, message = "验证码长度不能超过10个字符"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub captcha_code: Option<String>,

    /// 验证码ID
    #[validate(length(max = 100, message = "验证码ID长度不能超过100个字符"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub captcha_id: Option<String>,

    /// 用户协议同意
    #[validate(must_be_true(message = "必须同意用户协议"))]
    #[serde(default)]
    pub accept_terms: bool,

    /// 隐私政策同意
    #[validate(must_be_true(message = "必须同意隐私政策"))]
    #[serde(default)]
    pub accept_privacy: bool,

    /// 元数据
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<serde_json::Value>,
}

/// 刷新令牌请求
#[derive(Debug, Clone, Serialize, Deserialize, Validate, ToSchema)]
pub struct RefreshTokenRequest {
    /// 刷新令牌
    #[validate(
        length(min = 1, max = 1000, message = "刷新令牌不能为空"),
        custom(function = "validate_token_format")
    )]
    pub refresh_token: String,

    /// 设备ID
    #[validate(length(max = 100, message = "设备ID长度不能超过100个字符"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_id: Option<String>,

    /// 设备类型
    #[validate(length(max = 50, message = "设备类型长度不能超过50个字符"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_type: Option<String>,

    /// 用户代理
    #[validate(length(max = 500, message = "用户代理长度不能超过500个字符"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_agent: Option<String>,

    /// IP地址
    #[validate(length(max = 45, message = "IP地址长度不能超过45个字符"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip_address: Option<String>,
}

/// 忘记密码请求
#[derive(Debug, Clone, Serialize, Deserialize, Validate, ToSchema)]
pub struct ForgotPasswordRequest {
    /// 邮箱
    #[validate(
        length(max = 100, message = "邮箱长度不能超过100个字符"),
        email(message = "邮箱格式不正确")
    )]
    pub email: String,

    pub identifier: String,

    pub client_id: Option<String>,

    pub reset_url: Option<String>,

    pub expires_in_minutes: Option<u32>,

    /// 验证码
    #[validate(length(max = 10, message = "验证码长度不能超过10个字符"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub captcha: Option<String>,

    /// 验证码ID
    #[validate(length(max = 100, message = "验证码ID长度不能超过100个字符"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub captcha_id: Option<String>,
}

/// 重置密码请求
#[derive(Debug, Clone, Serialize, Deserialize, Validate, ToSchema)]
pub struct ResetPasswordRequest {
    /// 重置令牌
    #[validate(
        length(min = 1, max = 1000, message = "重置令牌不能为空"),
        custom(function = "validate_token_format")
    )]
    pub token: String,

    /// 新密码
    #[validate(
        length(min = 8, max = 128, message = "密码长度必须在8-128个字符之间"),
        custom(function = "validate_password_strength")
    )]
    pub new_password: String,

    /// 确认密码
    #[validate(must_match(other = "new_password", message = "两次输入的密码不一致"))]
    pub confirm_password: String,
}

/// 验证邮箱请求
#[derive(Debug, Clone, Serialize, Deserialize, Validate, ToSchema)]
pub struct VerifyEmailRequest {
    /// 验证令牌
    #[validate(
        length(min = 1, max = 1000, message = "验证令牌不能为空"),
        custom(function = "validate_token_format")
    )]
    pub token: String,

    /// 邮箱
    #[validate(
        length(max = 100, message = "邮箱长度不能超过100个字符"),
        email(message = "邮箱格式不正确")
    )]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
}

/// 重新发送验证邮件请求
#[derive(Debug, Clone, Serialize, Deserialize, Validate, ToSchema)]
pub struct ResendVerificationRequest {
    /// 邮箱
    #[validate(
        length(max = 100, message = "邮箱长度不能超过100个字符"),
        email(message = "邮箱格式不正确")
    )]
    pub email: String,

    /// 验证码
    #[validate(length(max = 10, message = "验证码长度不能超过10个字符"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub captcha: Option<String>,

    /// 验证码ID
    #[validate(length(max = 100, message = "验证码ID长度不能超过100个字符"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub captcha_id: Option<String>,
}

/// 登录方式
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, ToSchema)]
pub enum LoginMethod {
    #[serde(rename = "password")]
    Password,
    #[serde(rename = "sms")]
    Sms,
    #[serde(rename = "social")]
    Social,
    #[serde(rename = "mfa")]
    Mfa,
    #[serde(rename = "sso")]
    Sso,
}

impl Default for LoginMethod {
    fn default() -> Self {
        Self::Password
    }
}

fn default_login_method() -> LoginMethod {
    LoginMethod::Password
}

/// 验证用户标识符
fn validate_identifier(identifier: &str) -> Result<(), ValidationError> {
    if identifier.trim().is_empty() {
        return Err(ValidationError::new("identifier cannot be empty")
            .with_message(Cow::from("用户标识符不能为空")));
    }
    // 检查是否为邮箱格式
    static EMAIL_REGEX: Lazy<Regex> =
        Lazy::new(|| Regex::new(r"^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}$").unwrap());
    // 检查是否为手机号格式
    static PHONE_REGEX: Lazy<Regex> = Lazy::new(|| {
        Regex::new(r"^\+?[1-9]\d{1,14}$").unwrap() // E.164格式
    });
    // 检查是否为用户名格式
    static USERNAME_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(r"^[a-zA-Z0-9_]{3,50}$").unwrap());
    if EMAIL_REGEX.is_match(identifier)
        || PHONE_REGEX.is_match(identifier)
        || USERNAME_REGEX.is_match(identifier)
    {
        Ok(())
    } else {
        Err(ValidationError::new("invalid identifier format")
            .with_message(Cow::from("无效的用户标识符格式")))
    }
}

fn validate_password_format(password: &str) -> Result<(), ValidationError> {
    if password.contains(" ") {
        return Err(ValidationError::new("password cannot contain spaces")
            .with_message(Cow::from("密码不能包含空格")));
    }
    Ok(())
}

pub fn validate_username(username: &str) -> Result<(), ValidationError> {
    static USERNAME_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(r"^[a-zA-Z0-9_]{3,50}$").unwrap());
    if !USERNAME_REGEX.is_match(username) {
        return Err(ValidationError::new("invalid username format")
            .with_message(Cow::from("无效的用户名格式")));
    }
    //检查保留用户名
    let reserved_name = vec![
        "admin",
        "administrator",
        "root",
        "system",
        "support",
        "help",
        "info",
        "contact",
        "security",
        "noreply",
    ];

    if reserved_name.contains(&username.to_lowercase().as_str()) {
        return Err(ValidationError::new("reserved username")
            .with_message(Cow::from("该用户名已被保留,请更换用户名")));
    }
    Ok(())
}

pub fn validate_phone(phone: &str) -> Result<(), ValidationError> {
    if phone.is_empty() {
        return Ok(()); // 手机号可选
    }
    static PHONE_REGEX: Lazy<Regex> = Lazy::new(|| {
        // 支持国际格式和国内手机号
        Regex::new(r"^(?:\+?86)?1[3-9]\d{9}$|^\+[1-9]\d{1,14}$").unwrap()
    });
    if !PHONE_REGEX.is_match(phone) {
        return Err(ValidationError::new("invalid phone format")
            .with_message(Cow::from("无效的手机号格式")));
    }
    Ok(())
}

pub fn validate_password_strength(password: &str) -> Result<(), ValidationError> {
    if password.len() < 6 {
        return Err(ValidationError::new("password too short").with_message(Cow::from("密码太短")));
    }
    if password.len() > 128 {
        return Err(ValidationError::new("password too long").with_message(Cow::from("密码太长")));
    }

    // 检查包含的字符类型
    let has_lowercase = password.chars().any(|c| c.is_ascii_lowercase());
    let has_uppercase = password.chars().any(|c| c.is_ascii_uppercase());
    let has_digit = password.chars().any(|c| c.is_ascii_digit());
    let has_special = password.chars().any(|c| c.is_alphanumeric());

    //密码强度评估
    let mut strength_score = 0;
    if has_lowercase {
        strength_score += 1;
    }
    if has_uppercase {
        strength_score += 1;
    }
    if has_digit {
        strength_score += 1;
    }
    if has_special {
        strength_score += 1;
    }
    if password.len() >= 12 {
        strength_score += 1;
    }
    if strength_score < 3 {
        return Err(
            ValidationError::new("password too weak").with_message(Cow::from(
                "密码强度不足，建议包含大小写字母、数字和特殊字符",
            )),
        );
    }
    let common_passwords = vec![
        "password",
        "123456",
        "12345678",
        "qwerty",
        "abc123",
        "password1",
        "admin",
        "welcome",
        "monkey",
        "letmein",
    ];
    if common_passwords.contains(&password.to_lowercase().as_str()) {
        return Err(ValidationError::new("common password")
            .with_message(Cow::from("密码过于简单，请使用更复杂的密码")));
    }
    // 检查连续字符
    if contains_sequence(password) {
        return Err(
            ValidationError::new("sequence password").with_message(Cow::from("密码包含连续字符"))
        );
    }
    Ok(())
}

fn validate_token_format(token: &str) -> Result<(), ValidationError> {
    if token.trim().is_empty() {
        return Err(
            ValidationError::new("token_cannot_be_empty").with_message(Cow::from("令牌不能为空"))
        );
    }

    // 基本格式检查
    if token.len() < 10 || token.len() > 1000 {
        return Err(
            ValidationError::new("invalid_token_length").with_message(Cow::from("令牌长度不正确"))
        );
    }

    Ok(())
}

fn contains_sequence(password: &str) -> bool {
    if password.len() < 3 {
        return false;
    }
    let chars: Vec<char> = password.chars().collect();
    for i in 0..chars.len() - 2 {
        let c1 = chars[i] as u8;
        let c2 = chars[i + 1] as u8;
        let c3 = chars[i + 2] as u8;
        // 检查是否为连续字符
        if c1 + 1 == c2 && c2 + 1 == c3 {
            return true;
        }
        // 检查是否为连续字符的反向顺序
        if c1 - 1 == c2 && c2 - 1 == c3 {
            return true;
        }

        // 检查相同字符
        if c1 == c2 && c2 == c3 {
            return true;
        }
    }
    false
}
