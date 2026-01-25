// src/app/config/constants.rs
// 配置常量定义

/// 最小端口号
pub const MIN_PORT: u16 = 1;

/// 最大端口号
pub const MAX_PORT: u16 = 65535;

/// 默认HTTP端口
pub const DEFAULT_HTTP_PORT: u16 = 3000;

/// 默认gRPC端口
pub const DEFAULT_GRPC_PORT: u16 = 50051;

/// 默认健康检查端口
pub const DEFAULT_HEALTH_CHECK_PORT: u16 = 8080;

/// 默认数据库最大连接数
pub const DEFAULT_DB_MAX_CONNECTIONS: u32 = 10;

/// 默认数据库最小连接数
pub const DEFAULT_DB_MIN_CONNECTIONS: u32 = 2;

/// 默认Redis连接池大小
pub const DEFAULT_REDIS_POOL_SIZE: u32 = 10;

/// 默认JWT过期时间（分钟）
pub const DEFAULT_JWT_EXPIRY_MINUTES: u64 = 60;

/// 默认刷新令牌过期时间（天）
pub const DEFAULT_REFRESH_TOKEN_EXPIRY_DAYS: u64 = 7;

/// 默认密码哈希成本
pub const DEFAULT_PASSWORD_HASH_COST: u32 = 12;

/// 环境变量前缀
pub const DEFAULT_ENV_PREFIX: &str = "APP";

/// 环境变量分隔符
pub const DEFAULT_ENV_SEPARATOR: &str = "__";

/// 配置文件扩展名
pub const CONFIG_EXTENSIONS: [&str; 3] = ["toml", "yaml", "json"];

/// 有效的环境列表
pub const VALID_ENVIRONMENTS: [&str; 4] = ["development", "production", "staging", "test"];

/// 默认配置文件搜索路径
pub const CONFIG_SEARCH_PATHS: [&str; 6] = [
    "./config",
    "/etc/service-manager",
    "/usr/local/etc/service-manager",
    "$HOME/.config/service-manager",
    "$HOME/.service-manager",
    ".",
];

/// 默认日志级别
pub const DEFAULT_LOG_LEVEL: &str = "info";

/// 默认日志目录
pub const DEFAULT_LOG_DIR: &str = "./logs";

/// 默认日志保留天数
pub const DEFAULT_LOG_RETENTION_DAYS: u32 = 30;

/// 默认请求体限制（10MB）
pub const DEFAULT_BODY_LIMIT: usize = 10 * 1024 * 1024;

/// 默认请求超时时间（秒）
pub const DEFAULT_REQUEST_TIMEOUT_SECS: u64 = 30;

/// 默认数据库连接超时时间（秒）
pub const DEFAULT_DB_ACQUIRE_TIMEOUT_SECS: u64 = 10;

/// 默认数据库连接空闲超时时间（秒）
pub const DEFAULT_DB_IDLE_TIMEOUT_SECS: u64 = 600;

/// 默认数据库连接最大生命周期（秒）
pub const DEFAULT_DB_MAX_LIFETIME_SECS: u64 = 1800;

/// 默认Redis TTL（秒）
pub const DEFAULT_REDIS_TTL_SECS: u64 = 3600;

/// 默认TCP保活时间（秒）
pub const DEFAULT_TCP_KEEPALIVE_SECS: u64 = 60;

/// 默认HTTP/2保活间隔（秒）
pub const DEFAULT_HTTP2_KEEPALIVE_INTERVAL_SECS: u64 = 30;

/// 默认HTTP/2保活超时（秒）
pub const DEFAULT_HTTP2_KEEPALIVE_TIMEOUT_SECS: u64 = 5;
