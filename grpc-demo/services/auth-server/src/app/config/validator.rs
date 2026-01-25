// src/app/config/validator.rs
use std::path::Path;
use url::Url;
use validator::Validate;

use super::constants::*;
use super::error::ConfigError;

/// 配置验证器
pub struct ConfigValidator;

impl ConfigValidator {
    /// 验证应用配置
    pub fn validate(config: &super::AppConfig) -> Result<(), ConfigError> {
        // 基本验证
        config
            .validate()
            .map_err(|e| ConfigError::validation_error(e.to_string()))?;

        // 环境验证
        Self::validate_environment(&config.environment)?;

        // 端口验证
        Self::validate_ports(config)?;

        // URL验证
        Self::validate_urls(config)?;

        // 路径验证
        Self::validate_paths(config)?;

        // 生产环境安全检查
        if config.is_production() {
            Self::validate_production_safety(config)?;
        }

        Ok(())
    }

    /// 验证环境
    fn validate_environment(env: &str) -> Result<(), ConfigError> {
        let valid_envs = ["development", "production", "staging", "test"];

        if !valid_envs.contains(&env) {
            return Err(ConfigError::validation_error(format!(
                "Invalid environment '{}'. Must be one of: {}",
                env,
                valid_envs.join(", ")
            )));
        }

        Ok(())
    }

    /// 验证端口
    fn validate_ports(config: &super::AppConfig) -> Result<(), ConfigError> {
        // 检查HTTP端口是否在有效范围内
        if config.server.enabled {
            Self::validate_port_range(config.server.port, "server.port")?;
        }

        // 检查gRPC端口是否在有效范围内
        if config.grpc.enabled {
            Self::validate_port_range(config.grpc.port, "grpc.port")?;
        }

        // 检查健康检查端口
        if config.server.enable_health_check {
            Self::validate_port_range(config.server.health_check_port, "server.health_check_port")?;
        }

        // 检查端口冲突
        if config.server.enabled && config.grpc.enabled {
            if config.server.port == config.grpc.port {
                return Err(ConfigError::validation_error(
                    "HTTP and gRPC ports cannot be the same",
                ));
            }

            if config.server.enable_health_check
                && config.server.health_check_port == config.grpc.port
            {
                return Err(ConfigError::validation_error(
                    "Health check port conflicts with gRPC port",
                ));
            }
        }

        // 检查健康检查端口冲突
        if config.server.enabled && config.server.enable_health_check {
            if config.server.port == config.server.health_check_port {
                return Err(ConfigError::validation_error(
                    "HTTP port and health check port cannot be the same",
                ));
            }
        }

        Ok(())
    }

    /// 验证端口范围
    fn validate_port_range(port: u16, field: &str) -> Result<(), ConfigError> {
        if port < MIN_PORT || port > MAX_PORT {
            return Err(ConfigError::out_of_range(field, MIN_PORT, MAX_PORT));
        }

        // 避免使用已知的系统端口
        if port <= 1024 {
            tracing::warn!(
                "Port {} for {} is in the system port range (1-1024)",
                port,
                field
            );
        }

        Ok(())
    }

    /// 验证URL
    fn validate_urls(config: &super::AppConfig) -> Result<(), ConfigError> {
        // 验证数据库URL
        Self::validate_database_url(&config.database.url)?;

        // 验证Redis URL
        if let Some(redis) = &config.redis {
            Self::validate_redis_url(&redis.url)?;
        }

        // 验证RabbitMQ URL
        if let Some(rabbitmq) = &config.rabbitmq {
            if rabbitmq.enabled {
                Self::validate_rabbitmq_url(&rabbitmq.url)?;
            }
        }

        Ok(())
    }

    /// 验证数据库URL
    fn validate_database_url(url: &str) -> Result<(), ConfigError> {
        if url.is_empty() {
            return Err(ConfigError::missing_field("database.url"));
        }

        // 检查是否包含敏感信息
        if url.contains("password") && url.contains("changeme") {
            return Err(ConfigError::validation_error(
                "Database URL contains default password. Please change it.",
            ));
        }

        // 检查是否为本地数据库
        if config.is_production() && url.contains("localhost") {
            tracing::warn!("Using localhost database in production environment");
        }

        Ok(())
    }

    /// 验证Redis URL
    fn validate_redis_url(url: &str) -> Result<(), ConfigError> {
        if url.is_empty() {
            return Err(ConfigError::missing_field("redis.url"));
        }

        if !url.starts_with("redis://") && !url.starts_with("rediss://") {
            return Err(ConfigError::InvalidFormat(
                "Redis URL must start with redis:// or rediss://".to_string(),
            ));
        }

        Ok(())
    }

    /// 验证RabbitMQ URL
    fn validate_rabbitmq_url(url: &str) -> Result<(), ConfigError> {
        if url.is_empty() {
            return Err(ConfigError::missing_field("rabbitmq.url"));
        }

        if !url.starts_with("amqp://") && !url.starts_with("amqps://") {
            return Err(ConfigError::InvalidFormat(
                "RabbitMQ URL must start with amqp:// or amqps://".to_string(),
            ));
        }

        Ok(())
    }

    /// 验证路径
    fn validate_paths(config: &super::AppConfig) -> Result<(), ConfigError> {
        // 验证日志目录
        if config.logging.enable_file_logging {
            Self::validate_directory(&config.logging.log_dir, "logging.log_dir")?;
        }

        // 验证TLS证书路径
        if config.grpc.enable_tls {
            if let Some(cert_path) = &config.grpc.tls_cert_path {
                Self::validate_file_exists(cert_path, "grpc.tls_cert_path")?;
            }

            if let Some(key_path) = &config.grpc.tls_key_path {
                Self::validate_file_exists(key_path, "grpc.tls_key_path")?;
            }
        }

        Ok(())
    }

    /// 验证目录
    fn validate_directory(path: &str, field: &str) -> Result<(), ConfigError> {
        let path = Path::new(path);

        if !path.exists() {
            // 尝试创建目录
            if let Err(e) = std::fs::create_dir_all(path) {
                return Err(ConfigError::validation_error(format!(
                    "Failed to create directory {}: {}",
                    field, e
                )));
            }
        } else if !path.is_dir() {
            return Err(ConfigError::validation_error(format!(
                "{} is not a directory",
                field
            )));
        }

        Ok(())
    }

    /// 验证文件存在
    fn validate_file_exists(path: &str, field: &str) -> Result<(), ConfigError> {
        let path = Path::new(path);

        if !path.exists() {
            return Err(ConfigError::FileNotFound(
                path.to_string_lossy().to_string(),
            ));
        }

        if !path.is_file() {
            return Err(ConfigError::validation_error(format!(
                "{} is not a file",
                field
            )));
        }

        Ok(())
    }

    /// 验证生产环境安全性
    fn validate_production_safety(config: &super::AppConfig) -> Result<(), ConfigError> {
        let mut errors = Vec::new();

        // 检查JWT密钥
        if config.security.jwt_secret.len() < 32 {
            errors.push("JWT secret is too short for production (minimum 32 characters)");
        }

        if config.security.jwt_secret.contains("secret")
            || config.security.jwt_secret.contains("changeme")
        {
            errors.push("JWT secret contains insecure default values");
        }

        // 检查日志级别
        if config.logging.log_level == "debug" {
            errors.push("Debug logging is not recommended in production");
        }

        // 检查是否使用本地数据库
        if config.database.url.contains("localhost") {
            errors.push("Using localhost database in production is not recommended");
        }

        if !errors.is_empty() {
            return Err(ConfigError::validation_error(format!(
                "Production safety checks failed:\n{}",
                errors.join("\n")
            )));
        }

        Ok(())
    }
}
