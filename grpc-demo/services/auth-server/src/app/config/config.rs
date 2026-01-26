// src/app/config/config.rs
use crate::app::config::error::ConfigError;
use crate::app::config::validator::ConfigValidator;
use jsonwebtoken::Algorithm;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use validator::Validate;

/// 应用配置
#[derive(Debug, Clone, Serialize, Deserialize, Validate, ToSchema)]
pub struct AppConfig {
    #[validate(length(min = 1))]
    pub environment: String,
    pub server: ServerConfig,
    pub database: DatabaseConfig,
    pub redis: Option<RedisConfig>,
    /// gRPC配置
    #[validate(nested)]
    pub grpc: GrpcConfig,
    pub rabbitmq: Option<RabbitmqConfig>,
    pub logging: LogConfig,
    pub security: SecurityConfig,
    pub rate_limit: RateLimitConfig,
}

impl AppConfig {
    /// 统一验证入口
    ///
    /// 执行两阶段验证：
    /// 1. 字段级验证：使用 validator 派生宏验证单个字段
    /// 2. 业务逻辑验证：使用 ConfigValidator 验证跨字段逻辑
    pub fn validate(&self) -> Result<(), ConfigError> {
        // 第一阶段： 字段级验证(使用派生宏)
        Validate::validate(self)
            .map_err(|e| ConfigError::validation_error(format!("字段验证失败: {}", e)))?;
        // 第二阶段：业务逻辑验证（使用 ConfigValidator）
        ConfigValidator::validate(self)?;
        Ok(())
    }
    /// 判断是否为开发环境
    pub fn is_dev(&self) -> bool {
        self.environment == "dev"
    }
    /// 判断是否为生产环境
    pub fn is_prod(&self) -> bool {
        self.environment == "prod"
    }
    /// 判断是否为预发布环境
    pub fn is_staging(&self) -> bool {
        self.environment == "staging"
    }
    /// 判断是否为测试环境
    pub fn is_test(&self) -> bool {
        self.environment == "test"
    }
}

/// 服务器配置
#[derive(Debug, Clone, Serialize, Deserialize, Validate, ToSchema)]
pub struct ServerConfig {
    /// 是否启用
    pub enable_http: bool,
    /// 主机地址
    #[validate(length(min = 1, message = "主机地址不能为空"))]
    pub host: String,
    /// 端口
    #[validate(range(min = 1, max = 65535))]
    pub port: u16,

    /// 是否启用CORS
    pub enable_cors: bool,

    /// CORS允许的源
    pub cors_origins: Vec<String>,

    /// 请求超时时间(秒)
    #[validate(range(min = 1, max = 300, message = "请求超时时间必须在1-300秒之间"))]
    pub request_timeout_secs: u64,

    /// 请求体大小限制
    #[validate(range(
        min = 1024,
        max = 104857600,
        message = "请求体大小限制必须在1KB-100MB之间"
    ))] // 1KB- 100MB
    pub body_limit: usize,

    /// 是否启用健康检查
    pub enable_health_check: bool,

    /// 健康检查端口
    #[validate(range(min = 1, max = 65535, message = "健康检查端口必须在1-65535之间"))]
    pub health_check_port: u16,

    /// 是否启用混合模式
    pub enable_hybrid: bool,
}

/// gRPC配置
#[derive(Debug, Clone, Serialize, Deserialize, Validate, ToSchema)]
pub struct GrpcConfig {
    /// 是否启用
    pub enabled: bool,

    /// 主机地址
    #[validate(length(min = 1, message = "grpc地址不能为空"))]
    pub host: String,

    /// 端口
    #[validate(range(min = 1, max = 65535, message = "端口必须在1-65535之间"))]
    pub port: u16,

    /// 完整地址
    #[validate(length(min = 1))]
    pub address: String,

    /// 是否启用反射
    pub enable_reflection: bool,

    /// 是否启用TLS
    pub enable_tls: bool,

    /// TLS证书路径
    pub tls_cert_path: Option<String>,

    /// TLS密钥路径
    pub tls_key_path: Option<String>,

    /// 最大并发流
    #[validate(range(min = 1, max = 1000))]
    pub max_concurrent_streams: u32,

    /// 初始流窗口大小
    pub initial_stream_window_size: u32,

    /// 初始连接窗口大小
    pub initial_connection_window_size: u32,

    /// TCP保活时间（秒）
    pub tcp_keepalive_seconds: u64,

    /// TCP无延迟
    pub tcp_nodelay: bool,

    /// HTTP/2保活间隔（秒）
    pub http2_keepalive_interval_seconds: u64,

    /// HTTP/2保活超时（秒）
    pub http2_keepalive_timeout_seconds: u64,
}

/// 数据库配置
#[derive(Debug, Clone, Serialize, Deserialize, Validate, ToSchema)]
pub struct DatabaseConfig {
    /// 数据库URL
    #[validate(length(min = 1), url)]
    pub url: String,

    /// 最大连接数
    #[validate(range(min = 1, max = 100))]
    pub max_connections: u32,

    /// 最小连接数
    #[validate(range(min = 0, max = 50))]
    pub min_connections: u32,

    /// 获取连接超时时间（秒）
    #[validate(range(min = 1, max = 60))]
    pub acquire_timeout_secs: u64,

    /// 连接空闲超时时间（秒）
    pub idle_timeout_secs: u64,

    /// 连接最大生命周期（秒）
    pub max_lifetime_secs: u64,

    /// 是否运行迁移
    pub run_migrations: bool,

    /// 是否启用连接池健康检查
    pub enable_pool_health_check: bool,

    /// 健康检查间隔（秒）
    pub health_check_interval_secs: u64,
}

/// Redis配置
#[derive(Debug, Clone, Serialize, Deserialize, Validate, ToSchema)]
pub struct RedisConfig {
    /// Redis URL
    #[validate(length(min = 1))]
    pub url: String,

    /// 连接池大小
    #[validate(range(min = 1, max = 100))]
    pub pool_size: usize,

    /// 默认TTL（秒）
    pub default_ttl_secs: u64,

    /// 是否启用集群
    pub enable_cluster: bool,
}

impl Default for RedisConfig {
    fn default() -> Self {
        Self {
            url: "redis://127.0.0.1:6379".to_string(),
            pool_size: 10,
            default_ttl_secs: 10,
            enable_cluster: false,
        }
    }
}

/// RabbitMQ配置
#[derive(Debug, Clone, Serialize, Deserialize, Validate, ToSchema)]
pub struct RabbitmqConfig {
    /// 是否启用
    pub enabled: bool,

    /// RabbitMQ URL
    #[validate(length(min = 1))]
    pub url: String,

    /// 连接名称
    pub connection_name: String,

    /// 交换器名称
    pub exchange_name: String,

    /// 队列前缀
    pub queue_prefix: String,

    /// 预取数量
    pub prefetch_count: u16,
}

/// 日志配置
#[derive(Debug, Clone, Serialize, Deserialize, Validate, ToSchema)]
pub struct LogConfig {
    /// 日志级别
    #[validate(length(min = 1))]
    pub log_level: String,

    /// 日志目录
    #[validate(length(min = 1))]
    pub log_dir: String,

    /// 是否启用JSON格式
    pub enable_json_format: bool,

    /// 是否启用文件日志
    pub enable_file_logging: bool,

    /// 是否启用控制台日志
    pub enable_console_logging: bool,

    /// 日志保留天数
    pub retention_days: u32,
}

/// 安全配置
#[derive(Debug, Clone, Serialize, Deserialize, Validate, ToSchema)]
pub struct SecurityConfig {
    /// JWT密钥
    #[validate(length(min = 32))]
    pub jwt_secret: String,

    /// JWT过期时间（分钟）
    #[validate(range(min = 1, max = 1440))]
    pub jwt_expiry_minutes: u64,

    /// 刷新令牌过期时间（天）
    #[validate(range(min = 1, max = 30))]
    pub refresh_token_expiry_days: u64,

    /// 密码哈希成本
    #[validate(range(min = 8, max = 16))]
    pub password_hash_cost: u32,
}

/// 限流配置
#[derive(Debug, Clone, Serialize, Deserialize, Validate, ToSchema)]
pub struct RateLimitConfig {
    /// 是否启用
    pub enabled: bool,

    /// 每分钟请求数
    pub requests_per_minute: u32,

    /// 突发请求数
    pub burst_size: u32,

    /// 是否跳过认证
    pub skip_authentication: bool,
}

/// 配置摘要
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct ConfigSummary {
    pub environment: String,
    pub http_port: u16,
    pub grpc_port: u16,
    pub database_url: String,
    pub redis_url: Option<String>,
    pub has_message_queue: bool,
    pub config_files_count: usize,
    pub env_vars_count: usize,
}
