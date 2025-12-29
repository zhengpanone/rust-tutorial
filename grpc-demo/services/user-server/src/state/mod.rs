use crate::config::Config;
use crate::db::postgres::create_pg_pool;
use crate::db::redis::create_redis_pool;
use crate::grpc::user::GrpcUserService;
use crate::service::user_service::UserService;
use anyhow::{Error, anyhow};
use deadpool_redis::Pool as RedisPool;
use sqlx::PgPool;
use std::sync::Arc;
use tracing::log::info;
use crate::grpc::hello::GrpcHelloService;

#[derive(Clone)]
pub struct AppState {
    pub db: PgPool,
    pub redis_pool: RedisPool,
    pub config: Config,
}

impl AppState {
    pub async fn new(config: Config) -> Result<Self, Error> {
        // 初始化数据库
        let db_pool = create_pg_pool(&config.database)
            .await
            .expect("Failed to connect to DB");
        info!("Database connection established");
        // 初始化redis
        let redis_pool = create_redis_pool(&config.redis)
            .await
            .map_err(|e| anyhow!("Failed to connect to Redis: {}", e))?;

        // 运行数据库迁移
        // sqlx::migrate!("./migrations").run(&db_pool).await?;
        Ok(Self {
            db: db_pool,
            redis_pool,
            config,
        })
    }
}

#[derive(Clone)]
pub struct App {
   pub state: Arc<AppState>,
   pub user_service: Arc<UserService>,
    // TODO
    // role_service: Arc<RoleService>,
}

impl App {
    pub async fn new(config: Config) -> Result<Self, Error> {
        let state = Arc::new(AppState::new(config).await?);
        Ok(Self {
            state: state.clone(),
            user_service: Arc::new(UserService::new(state.clone())),
        })
    }

    pub fn grpc_user_service(&self) -> GrpcUserService {
        GrpcUserService::new(self.user_service.clone())
    }
    pub fn grpc_hello_service(&self) -> GrpcHelloService {
        GrpcHelloService::default()
    }
}
