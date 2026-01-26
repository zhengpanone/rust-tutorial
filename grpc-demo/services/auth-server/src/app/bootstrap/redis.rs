use crate::app::config::config::RedisConfig;
use common::error::AppResult;
use deadpool_redis::redis::AsyncCommands;
use deadpool_redis::{Config, PoolConfig};
use redis::Client as RedisClient;
use std::sync::Arc;
use tracing::{error, info, warn};

pub async fn init_redis(
    config: &Option<RedisConfig>,
) -> AppResult<(Option<Arc<RedisClient>>, Option<Arc<deadpool_redis::Pool>>)> {
    match config {
        Some(cfg) => {
            info!("Initializing redis connection pool...");
            // 创建单独的 Client
            let client = RedisClient::open(cfg.url.clone())?;

            // 配置 deadpool-redis
            let pool_config = Config::from_url(cfg.url.clone());

            // 创建连接池
            let pool = pool_config
                .create_pool(Some(deadpool_redis::Runtime::Tokio1))
                .map_err(|e| {
                    error!("Failed to create Redis connection pool: {}", e);
                    e
                })?;

            // 测试 Redis 连接
            test_redis_connection(&pool).await?;

            info!("Redis connection pool initialized successfully");
            Ok((Some(Arc::new(client)), Some(Arc::new(pool))))
        }
        None => {
            warn!("Redis is not configured, skipping initialization");
            Ok((None, None))
        }
    }
}

/// 测试 Redis 连接
async fn test_redis_connection(pool: &deadpool_redis::Pool) -> AppResult<()> {
    let mut conn = pool.get().await?;
    let _: String = conn.ping().await?;
    Ok(())
}
