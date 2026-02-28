use common::error::{AppError, AppResult};
use serde::{Deserialize, Serialize};
use std::fmt;

/// 邮箱值对象
/// 封装邮箱验证规则，确保所有邮箱都是有效的
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Email(String);

impl Email {
    /// 创建新的邮箱值对象
    /// 会自动转换为小写并验证格式
    pub fn new(email: &str) -> AppResult<Self> {
        let email = email.trim().to_lowercase();
        if email.is_empty() {
            return Err(AppError::Validation("邮箱不能为空".to_string()));
        }
        // 验证邮箱格式
        if !email.contains('@') {
            return Err(AppError::Validation("无效的邮箱格式".into()));
        }
        Ok(Self(email))
    }
    /// 获取邮箱字符串引用
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// 获取邮箱域名部分
    pub fn domain(&self) -> &str {
        self.0.split('@').nth(1).unwrap_or("")
    }
    /// 获取邮箱用户名部分（@之前）
    pub fn local_part(&self) -> &str {
        self.0.split('@').next().unwrap_or("")
    }
}

impl AsRef<str> for Email {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for Email {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<Email> for String {
    fn from(email: Email) -> Self {
        email.0
    }
}

impl TryFrom<&str> for Email {
    type Error = AppError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}

impl TryFrom<String> for Email {
    type Error = AppError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::new(&value)
    }
}
