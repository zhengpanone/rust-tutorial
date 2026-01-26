// src/app/config/error.rs
use config::ConfigError as RawConfigError;
use serde::{Deserialize, Serialize};
use std::fmt;
use thiserror::Error;
use utoipa::ToSchema;

/// 配置错误类型
#[derive(Debug, Error, Serialize, Deserialize, ToSchema)]
pub enum ConfigError {
    /// 文件不存在错误
    #[error("配置文件不存在: {0}")]
    FileNotFound(String),

    /// 文件读取错误
    #[error("无法读取配置文件: {0}")]
    FileReadError(String),

    /// 文件解析错误
    #[error("配置文件解析失败: {0}")]
    ParseError(String),

    /// 环境变量错误
    #[error("环境变量设置错误: {0}")]
    EnvVarError(String),

    /// 配置值缺失错误
    #[error("配置项缺失: {0}")]
    MissingField(String),

    /// 配置值类型错误
    #[error("配置项类型错误: {0}")]
    InvalidType(String),

    /// 配置值验证错误
    #[error("配置项验证失败: {0}")]
    ValidationError(String),

    /// 配置值范围错误
    #[error("配置项值超出范围: {0}")]
    OutOfRange(String),

    /// 配置值格式错误
    #[error("配置项格式错误: {0}")]
    InvalidFormat(String),

    /// 未知错误
    #[error("未知配置错误: {0}")]
    Unknown(String),
}

impl ConfigError {
    /// 创建文件不存在错误
    pub fn file_not_found(path: impl Into<String>) -> Self {
        Self::FileNotFound(path.into())
    }

    /// 创建配置项缺失错误
    pub fn missing_field(field: impl Into<String>) -> Self {
        Self::MissingField(field.into())
    }

    /// 创建验证错误
    pub fn validation_error(message: impl Into<String>) -> Self {
        Self::ValidationError(message.into())
    }

    /// 创建范围错误
    pub fn out_of_range(
        field: impl Into<String>,
        min: impl fmt::Display,
        max: impl fmt::Display,
    ) -> Self {
        Self::OutOfRange(format!("{} 必须在 {} 和 {} 之间", field.into(), min, max))
    }

    /// 获取错误码
    pub fn error_code(&self) -> &'static str {
        match self {
            ConfigError::FileNotFound(_) => "CONFIG_FILE_NOT_FOUND",
            ConfigError::FileReadError(_) => "CONFIG_FILE_READ_ERROR",
            ConfigError::ParseError(_) => "CONFIG_PARSE_ERROR",
            ConfigError::EnvVarError(_) => "CONFIG_ENV_VAR_ERROR",
            ConfigError::MissingField(_) => "CONFIG_MISSING_FIELD",
            ConfigError::InvalidType(_) => "CONFIG_INVALID_TYPE",
            ConfigError::ValidationError(_) => "CONFIG_VALIDATION_ERROR",
            ConfigError::OutOfRange(_) => "CONFIG_OUT_OF_RANGE",
            ConfigError::InvalidFormat(_) => "CONFIG_INVALID_FORMAT",
            ConfigError::Unknown(_) => "CONFIG_UNKNOWN_ERROR",
        }
    }

    /// 获取错误严重级别
    pub fn severity(&self) -> ConfigErrorSeverity {
        match self {
            ConfigError::FileNotFound(_) => ConfigErrorSeverity::Fatal,
            ConfigError::MissingField(field) if is_critical_field(field) => {
                ConfigErrorSeverity::Fatal
            }
            ConfigError::MissingField(_) => ConfigErrorSeverity::Error,
            ConfigError::ValidationError(_) => ConfigErrorSeverity::Error,
            ConfigError::OutOfRange(_) => ConfigErrorSeverity::Warning,
            ConfigError::InvalidFormat(_) => ConfigErrorSeverity::Warning,
            _ => ConfigErrorSeverity::Error,
        }
    }
}

/// 检查是否为关键配置字段
fn is_critical_field(field: &str) -> bool {
    matches!(
        field,
        "database.url" | "security.jwt_secret" | "server.port" | "grpc.port"
    )
}

/// 配置错误严重级别
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, ToSchema)]
pub enum ConfigErrorSeverity {
    /// 致命错误 - 应用无法启动
    Fatal = 3,
    /// 错误 - 功能可能受限
    Error = 2,
    /// 警告 - 不影响主要功能
    Warning = 1,
    /// 信息 - 仅记录
    Info = 0,
}

impl fmt::Display for ConfigErrorSeverity {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ConfigErrorSeverity::Fatal => write!(f, "FATAL"),
            ConfigErrorSeverity::Error => write!(f, "ERROR"),
            ConfigErrorSeverity::Warning => write!(f, "WARNING"),
            ConfigErrorSeverity::Info => write!(f, "INFO"),
        }
    }
}

/// 从底层ConfigError转换
impl From<RawConfigError> for ConfigError {
    fn from(error: RawConfigError) -> Self {
        match error {
            RawConfigError::FileParse { cause, uri } => {
                let msg = if let Some(uri) = uri {
                    format!("文件 {} 解析失败: {}", uri, cause)
                } else {
                    format!("配置文件解析失败: {}", cause)
                };
                ConfigError::ParseError(msg)
            }
            RawConfigError::Message(msg) => ConfigError::ParseError(msg),
            RawConfigError::NotFound(msg) => ConfigError::MissingField(msg),
            RawConfigError::Type { .. } => ConfigError::InvalidType(error.to_string()),
            RawConfigError::Foreign(_) => ConfigError::Unknown(error.to_string()),
            // 添加处理新增的变体
            RawConfigError::Frozen => ConfigError::Unknown("配置已被冻结，无法修改".to_string()),
            // 处理未来可能新增的变体
            // 抑制不可达模式警告
            #[allow(unreachable_patterns)]
            _ => ConfigError::Unknown(format!("未知配置错误: {}", error.to_string())),
        }
    }
}

/// 从std::io::Error转换
impl From<std::io::Error> for ConfigError {
    fn from(error: std::io::Error) -> Self {
        match error.kind() {
            std::io::ErrorKind::NotFound => ConfigError::FileNotFound(error.to_string()),
            _ => ConfigError::FileReadError(error.to_string()),
        }
    }
}

/// 从serde_yaml::Error转换
impl From<serde_yaml::Error> for ConfigError {
    fn from(error: serde_yaml::Error) -> Self {
        ConfigError::ParseError(error.to_string())
    }
}

/// 从toml::de::Error转换
impl From<toml::de::Error> for ConfigError {
    fn from(error: toml::de::Error) -> Self {
        ConfigError::ParseError(error.to_string())
    }
}

/// 从json::Error转换
impl From<serde_json::Error> for ConfigError {
    fn from(error: serde_json::Error) -> Self {
        ConfigError::ParseError(error.to_string())
    }
}
