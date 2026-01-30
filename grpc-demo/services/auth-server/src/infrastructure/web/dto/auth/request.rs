use once_cell::sync::Lazy;
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;
use utoipa::ToSchema;
use validator::{Validate, ValidationError};

// src/infrastructure/web/dto/auth/requests.rs
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
    // TODO
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
