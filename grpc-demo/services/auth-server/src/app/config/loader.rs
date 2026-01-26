use crate::app::config::config::AppConfig;
use crate::app::config::constants::{DEFAULT_ENV_PREFIX, DEFAULT_ENV_SEPARATOR};
use crate::app::config::error::ConfigError;
use config::{Config, Environment, File, FileFormat};
use std::env;
use std::path::Path;
use tracing::{debug, warn};

// src/app/config/loader.rs
/// 配置加载器
pub struct ConfigLoader;

impl ConfigLoader {
    /// 从默认路径加载配置
    ///
    /// 按顺序尝试：
    /// 1. 当前目录下的 config 文件夹
    /// 2. 环境变量
    /// 3. 默认配置

    pub fn load() -> Result<AppConfig, ConfigError> {
        debug!("cwd = {:?}", std::env::current_dir());
        // 加载环境变量
        dotenv::from_path(concat!(env!("CARGO_MANIFEST_DIR"), "/.env")).ok();
        let environment = env::var("APP_ENVIRONMENT").unwrap_or_else(|_| "dev".to_string());

        Self::load_with_environment(&environment)
    }
    /// 从指定环境加载配置
    pub fn load_with_environment(environment: &str) -> Result<AppConfig, ConfigError> {
        let mut builder = Config::builder();

        // 1. 加载基础配文件
        let config_file = format!("config/config.{}", Self::detect_config_format()?);
        // let absolute_path = Path::new(&config_file).canonicalize()?;
        // debug!("config_file = {:?}", absolute_path);
        if Path::new(&config_file).exists() {
            builder = builder.add_source(File::with_name(&config_file))
        } else {
            // 如果没有配置文件，则使用默认配置
            warn!("No config file found, using defaults");
        }
        // 2. 加载环境特定配置
        let env_config_file = format!(
            "config/config.{}.{}",
            environment,
            Self::detect_config_format()?
        );
        if Path::new(&env_config_file).exists() {
            builder = builder.add_source(File::with_name(&env_config_file).required(false))
        }

        // 3. 加载环境变量覆盖
        builder = builder.add_source(
            Environment::with_prefix(DEFAULT_ENV_PREFIX)
                .prefix_separator(DEFAULT_ENV_SEPARATOR)
                .separator(DEFAULT_ENV_SEPARATOR)
                .try_parsing(true),
        );

        // 4. 构建配置
        let config = builder.build()?;

        // 5. 反序列化为AppConfig
        let mut app_config: AppConfig = config.try_deserialize()?;

        // 6. 设置环境变量
        app_config.environment = environment.to_string();
        Ok(app_config)
    }

    /// 从指定文件加载配置
    pub fn load_from_file(path: &str) -> Result<AppConfig, ConfigError> {
        let config = Config::builder()
            .add_source(File::with_name(path))
            .add_source(
                Environment::with_prefix(DEFAULT_ENV_PREFIX)
                    .prefix_separator(DEFAULT_ENV_SEPARATOR)
                    .separator(DEFAULT_ENV_SEPARATOR)
                    .try_parsing(true),
            )
            .build()?;

        let mut app_config: AppConfig = config.try_deserialize()?;
        // 设置环境变量
        app_config.environment =
            env::var("APP_ENVIRONMENT").unwrap_or_else(|_| "dev".to_string());
        Ok(app_config)
    }
    /// 从字符串加载配置
    pub fn load_from_string(content: &str, format: &str) -> Result<AppConfig, ConfigError> {
        let file_format = match format {
            "toml" => FileFormat::Toml,
            "yaml" | "yml" => FileFormat::Yaml,
            "json" => FileFormat::Json,
            _ => {
                return Err(ConfigError::InvalidFormat(format!(
                    "Unsupported config format: {}",
                    format
                )));
            }
        };

        let source = File::from_str(content, file_format);

        let config = Config::builder().add_source(source).build()?;

        let mut app_config: AppConfig = config.try_deserialize()?;

        app_config.environment =
            env::var("APP_ENVIRONMENT").unwrap_or_else(|_| "development".to_string());
        Ok(app_config)
    }
    /// 应用环境变量覆盖
    pub fn apply_env_override(mut config: AppConfig) -> Result<AppConfig, ConfigError> {
        // 覆盖数据库URL
        if let Ok(db_url) = env::var("DATABASE_URL") {
            config.database.url = db_url;
        }
        // 覆盖JWT秘钥
        if let Ok(jwt_secret) = env::var("JWT_SECRET") {
            config.security.jwt_secret = jwt_secret;
        }
        // 覆盖RedisURL
        if let Ok(redis_url) = env::var("REDIS_URL") {
            if let Some(ref mut redis) = config.redis {
                redis.url = redis_url;
            }
        }
        // 覆盖HTTP服务地址
        if let Ok(port) = env::var("HTTP_SERVER_ADDRESS") {
            if let Ok(port_num) = port.parse::<u16>() {
                config.server.port = port_num;
            }
        }

        // 覆盖gRPC服务地址
        if let Ok(port) = env::var("GRPC_SERVER_ADDRESS") {
            if let Ok(port_num) = port.parse::<u16>() {
                config.grpc.port = port_num;
            }
        }
        Ok(config)
    }

    /// 检测配置文件格式
    fn detect_config_format() -> Result<String, ConfigError> {
        for ext in ["toml", "yaml", "yml", "json"] {
            let file = format!("config/config.{}", ext);
            if Path::new(&file).exists() {
                return Ok(ext.to_string());
            }
        }
        // 默认使用 toml
        Ok("toml".to_string())
    }
    /// 验证配置并加载
    pub fn load_and_validate() -> Result<AppConfig, ConfigError> {
        let config = Self::load()?;
        config.validate()?;
        Ok(config)
    }
}
