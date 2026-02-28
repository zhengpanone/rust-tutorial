use common::error::{AppError, AppResult};
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::fmt;
use std::sync::LazyLock;

/// 用户名验证正则：只允许字母、数字、下划线，不能以数字开头
static USERNAME_PATTERN: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"^[a-zA-Z_][a-zA-Z0-9_]{2,19}$").expect("Invalid username regex"));

/// 用户名值对象
/// 封装用户名验证规则：
/// - 长度 3-20 个字符
/// - 只允许字母、数字、下划线
/// - 不能以数字开头
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Username(String);

impl Username {
    /// 最小长度
    pub const MIN_LENGTH: usize = 3;
    /// 最大长度
    pub const MAX_LENGTH: usize = 20;

    /// 创建新的用户名值对象
    pub fn new(username: &str) -> AppResult<Self> {
        let username = username.trim();

        // 长度验证
        if username.len() < Self::MIN_LENGTH {
            return Err(AppError::Validation(format!(
                "用户名长度不能少于 {} 个字符",
                Self::MIN_LENGTH
            )));
        }

        if username.len() > Self::MAX_LENGTH {
            return Err(AppError::Validation(format!(
                "用户名长度不能超过 {} 个字符",
                Self::MAX_LENGTH
            )));
        }

        // 格式验证
        if !USERNAME_PATTERN.is_match(username) {
            return Err(AppError::Validation(
                "用户名只能包含字母、数字和下划线，且不能以数字开头".to_string(),
            ));
        }

        Ok(Self(username.to_string()))
    }

    /// 获取用户名字符串引用
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// 检查是否为保留用户名（如 admin, system 等）
    pub fn is_reserved(&self) -> bool {
        matches!(
            self.0.to_lowercase().as_str(),
            "admin" | "administrator" | "system" | "root" | "superuser" | "moderator" | "mod"
        )
    }
}

impl AsRef<str> for Username {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for Username {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<Username> for String {
    fn from(username: Username) -> Self {
        username.0
    }
}

impl TryFrom<&str> for Username {
    type Error = AppError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}

impl TryFrom<String> for Username {
    type Error = AppError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::new(&value)
    }
}
