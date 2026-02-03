use std::sync::Arc;
use deadpool_redis::Pool as RedisPool;
use common::error::AppResult;
use crate::app::config::config::AppConfig;

#[derive(Clone)]
pub struct AppServices{
    pub pg_pool: sqlx::PgPool,
    pub redis_pool: Arc<RedisPool>,
}

impl AppServices{
    pub async fn new(config: &AppConfig)->AppResult<Self>{
        todo!()
    }
}