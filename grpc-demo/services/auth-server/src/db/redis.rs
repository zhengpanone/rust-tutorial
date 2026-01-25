use crate::config::database::RedisConfig;
use deadpool_redis::Pool as RedisPool;
use deadpool_redis::{Config, CreatePoolError};

pub async fn create_redis_pool(redis_config: &RedisConfig) -> Result<RedisPool, CreatePoolError> {
    // 创建连接池
    let cfg = Config::from_url(&redis_config.url);
    cfg.create_pool(Some(deadpool_redis::Runtime::Tokio1))
}
