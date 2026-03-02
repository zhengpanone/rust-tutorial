use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use validator::Validate;

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

impl Default for DatabaseConfig {
    fn default() -> Self {
        Self {
            url: "postgres://postgres:postgres@127.0.0.1:30432/gmall".to_string(),
            // url: "postgres://postgres:postgres@127.0.0.1:15432/gmall".to_string(),
            max_connections: 20,
            min_connections: 5,
            acquire_timeout_secs: 0,
            idle_timeout_secs: 0,
            max_lifetime_secs: 0,
            run_migrations: false,
            enable_pool_health_check: false,
            health_check_interval_secs: 0,
        }
    }
}
