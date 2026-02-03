// src/app/config/config.rs

use super::{
    database::DatabaseConfig, features::FeaturesConfig, message_queue::MessageQueueConfig,
    server::ServerConfig,
};
use crate::app::config::error::ConfigError;
pub use crate::app::config::security::SecurityConfig;
use crate::app::config::validator::ConfigValidator;
use anyhow::Context;
use common::error::AppError;
use config::{Config, Environment, File, FileFormat};
use directories::ProjectDirs;
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};
use tracing::{debug, info, warn};
use utoipa::ToSchema;
use validator::Validate;

/// 应用配置
#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct AppConfig {
    /// 应用名称
    pub name: String,

    /// 应用版本
    pub version: String,

    #[validate(length(min = 1))]
    pub environment: String,

    /// 配置文件路径
    #[serde(skip)]
    pub config_path: PathBuf,

    /// 配置目录
    #[serde(skip)]
    pub config_dir: PathBuf,

    /// 临时目录
    #[serde(skip)]
    pub temp_dir: PathBuf,

    /// 功能配置
    pub features: FeaturesConfig,

    /// 是否启用调试模式
    pub debug: bool,

    pub server: ServerConfig,
    pub database: DatabaseConfig,

    pub logging: LogConfig,

    pub redis: Option<RedisConfig>,

    /// gRPC配置
    #[validate(nested)]
    pub grpc: GrpcConfig,

    pub security: SecurityConfig,

    pub rate_limit: RateLimitConfig,
    // pub message_queue: Option<MessageQueueConfig>,
}

impl AppConfig {
    /// 加载应用配置
    pub fn load() -> Result<Self, AppError> {
        info!("Loading application  configuration ");
        // 确定配置文件路径
        let config_paths = determine_config_paths();

        // 创建配置构建器
        let mut builder = Config::builder();

        // 添加默认配置
        builder = builder.add_source(
            File::from_str(
                include_str!("../../../config/default.toml"),
                FileFormat::Toml,
            )
            .required(false),
        );

        // 添加环境特定的配置文件
        for path in config_paths.iter().rev() {
            if path.exists() {
                info!("📁 Loading configuration file: {}", path.display());
                builder = builder.add_source(File::from(path.clone()).required(false));
            } else {
                debug!("📁 Configuration file not found: {}", path.display());
            }
        }

        // 添加环境变量
        builder = builder.add_source(
            Environment::with_prefix("APP")
                .separator("__")
                .list_separator(",")
                .try_parsing(true),
        );

        // 添加命令行参数
        #[cfg(feature = "clap")]
        {
            builder =
                builder.add_source(config::Environment::with_prefix("APP").prefix_separator("_"));
        }

        // 构建配置
        let config = match builder.build() {
            Ok(cfg) => cfg,
            Err(e) => {
                warn!("Failed to build configuration: {}, using defaults", e);
                info!("✅ Configuration loaded successfully (using defaults)");
                return Ok(Self::default());
            }
        };

        // 反序列化配置
        let mut app_config: Self = match config.try_deserialize() {
            Ok(cfg) => cfg,
            Err(e) => {
                warn!(
                    "Failed to deserialize configuration from file: {}, using defaults",
                    e
                );
                info!("✅ Configuration loaded successfully (using defaults)");
                return Ok(Self::default());
            }
        };

        // 设置配置路径
        if let Some(first_path) = config_paths.iter().find(|p| p.exists()) {
            app_config.config_path = first_path.clone();
            app_config.config_dir = first_path
                .parent()
                .unwrap_or_else(|| Path::new("."))
                .to_path_buf();
        }

        // 创建必要的目录
        create_necessary_directories(&app_config)?;

        // 验证配置
        match app_config.validate() {
            Ok(_) => {}
            Err(e) => {
                warn!("Configuration validation failed: {}, using defaults", e);
                info!("✅ Configuration loaded successfully (using defaults)");
                return Ok(Self::default());
            }
        }

        // 记录加载的配置
        log_configuration(&app_config);

        info!("✅ Configuration loaded successfully");
        Ok(app_config)
    }

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
        self.environment == "dev" || self.environment == "development"
    }
    /// 判断是否为生产环境
    pub fn is_prod(&self) -> bool {
        self.environment == "prod" || self.environment == "production"
    }
    /// 判断是否为预发布环境
    pub fn is_staging(&self) -> bool {
        self.environment == "staging" || self.environment == "stage"
    }
    /// 判断是否为测试环境
    pub fn is_test(&self) -> bool {
        self.environment == "test" || self.environment == "testing"
    }

    /// 获取是否启用调试模式
    pub fn is_debug(&self) -> bool {
        self.debug || self.is_dev()
    }
}

fn determine_config_paths() -> Vec<PathBuf> {
    let mut paths = Vec::new();

    // 1. 从环境变量中获取配置路径
    if let Ok(env_path) = std::env::var("APP_CONFIG_PATH") {
        paths.push(PathBuf::from(env_path));
    }
    // 2. 从命令参数获取配置路径
    #[cfg(feature = "clap")]
    {
        use clap::Parser;
        struct Args {
            #[arg(long, value = "FILE")]
            config: Option<PathBuf>,
        }

        if let Ok(args) = Args::try_parse() {
            if let Some(config_path) = args.config {
                paths.push(config_path);
            }
        }
    }
    // 3. 从当前工作目录中获取
    paths.push(PathBuf::from("config/default.toml"));
    // 4. 从环境特定的配置文件获取
    if let Ok(env) = std::env::var("APP_ENVIRONMENT") {
        let env_config_path = format!("config/{}.toml", env.to_lowercase());
        paths.push(PathBuf::from(env_config_path));

        // 本地环境配置
        let local_env_config_path = format!("config/{}.local.toml", env.to_lowercase());
        paths.push(PathBuf::from(local_env_config_path));
    }
    // 5. 本地配置文件(开发使用)
    paths.push(PathBuf::from("config/local.toml"));

    // 6. 从系统配置目录获取
    if let Some(proj_dirs) = ProjectDirs::from("com", "microservice", "manager") {
        let sys_config_path = proj_dirs.config_dir().join("config.toml");
        paths.push(sys_config_path);
    }
    // 7. 用户主目录配置
    if let Some(home_dir) = dirs_next::home_dir() {
        let home_config_path = home_dir.join(".microservice-manager/config.toml");
        paths.push(home_config_path);
    }

    // 8. 内嵌默认配置
    paths.push(PathBuf::from("default.toml"));

    info!("Config file search paths:");
    for (i, path) in paths.iter().enumerate() {
        info!(" {}. {}", i + 1, path.display());
    }
    paths
}

// 创建必要的目录
fn create_necessary_directories(config: &AppConfig) -> Result<(), AppError> {
    let directories = vec![&config.temp_dir];

    for dir in directories {
        if !dir.exists() {
            info!("📁 Creating directory: {}", dir.display());
            std::fs::create_dir_all(dir)
                .with_context(|| format!("Failed to create directory: {}", dir.display()))
                .unwrap();
        }
    }

    Ok(())
}

/// 记录配置信息
fn log_configuration(config: &AppConfig) {
    info!("📋 Application configuration:");
    info!("  Name: {}", config.name);
    info!("  Version: {}", config.version);
    info!("  Environment: {}", config.environment);
    info!("  Config path: {}", config.config_path.display());
    info!("  Debug mode: {}", config.is_debug());

    // 敏感信息不记录
    debug!("📋 Server configuration: {:#?}", config.server);
    debug!("📋 Database configuration: {:#?}", config.database);
    // debug!("📋 Redis configuration: {:#?}", config.redis);
    // debug!("📋 Security configuration: {:#?}", config.security);

    // 记录功能开关
    info!("🎚️ Feature flags:");
    info!(
        "  User registration: {}",
        config.features.enable_user_registration
    );
    info!(
        "  Email verification: {}",
        config.features.enable_email_verification
    );
    info!(
        "  API rate limiting: {}",
        config.features.enable_api_rate_limiting
    );
    info!("  Audit logging: {}", config.features.enable_audit_logging);

    // 记录启用的服务
    info!("🌐 Enabled services:");
    info!("  HTTP server: {}", config.server.enable_http);
    // info!("  gRPC server: {}", config.grpc.enabled);
    // info!("  Redis: {:?}", config.redis);
    // info!("  Message queue: {:?}", config.message_queue);
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
    // pub has_message_queue: bool,
    pub config_files_count: usize,
    pub env_vars_count: usize,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            name: "Auth-Server".to_string(),
            version: env!("CARGO_PKG_VERSION").to_string(),
            environment: "development".to_string(),
            config_path: PathBuf::from("config/default.toml"),
            config_dir: PathBuf::from("config"),
            temp_dir: Default::default(),
            features: Default::default(),
            debug: false,
            server: ServerConfig::default(),
            database: DatabaseConfig::default(),
            logging: LogConfig::default(),
            redis: Some(RedisConfig::default()),
            security: SecurityConfig::default(),
            rate_limit: RateLimitConfig::default(),
            grpc: GrpcConfig::default(),
            // message_queue: None,
        }
    }
}

impl Default for RedisConfig {
    fn default() -> Self {
        Self {
            url: "redis://:redis123456@127.0.0.1:30379".to_string(),
            pool_size: 10,
            default_ttl_secs: 10,
            enable_cluster: false,
        }
    }
}

impl Default for RateLimitConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            requests_per_minute: 0,
            burst_size: 0,
            skip_authentication: false,
        }
    }
}

impl Default for LogConfig {
    fn default() -> Self {
        Self {
            log_level: "".to_string(),
            log_dir: "".to_string(),
            enable_json_format: false,
            enable_file_logging: false,
            enable_console_logging: false,
            retention_days: 0,
        }
    }
}

impl Default for GrpcConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            host: "grpc://127.0.0.1:18080".to_string(),
            port: 8080,
            address: "11111".to_string(),
            enable_reflection: false,
            enable_tls: false,
            tls_cert_path: None,
            tls_key_path: None,
            max_concurrent_streams: 10,
            initial_stream_window_size: 10,
            initial_connection_window_size: 10,
            tcp_keepalive_seconds: 10,
            tcp_nodelay: false,
            http2_keepalive_interval_seconds: 10,
            http2_keepalive_timeout_seconds: 10,
        }
    }
}
