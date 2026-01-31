// use crate::config::Config;
// use crate::db::postgres::create_pg_pool;
// use crate::db::redis::create_redis_pool;
// use crate::grpc::hello::GrpcHelloService;
// use crate::grpc::user::GrpcUserService;
// use crate::services::user_service::UserService;
use crate::app::bootstrap::server::jwt::JwtService;
use crate::app::config::config::AppConfig;
use anyhow::{Error, anyhow};
use deadpool_redis::Pool as RedisPool;
use lapin::{Channel, Connection as RabbitmqConnection};
use sqlx::PgPool;
use std::sync::Arc;
use tracing::log::info;
use crate::application::services::auth_app::AuthApp;
use crate::application::services::user_app::UserApp;

#[derive(Clone)]
pub struct AppState {
    /// 配置
    pub config: Arc<AppConfig>,
    /// 数据库池
    pub db_pool: PgPool,
    /// Redis连接池
    pub redis_pool: Option<RedisPool>,

    /// RabbitMQ连接
    pub rabbitmq_connection: Option<Arc<RabbitmqConnection>>,
    pub rabbitmq_channel: Option<Arc<Channel>>,

    /// 安全服务
    pub jwt_service: Arc<JwtService>,
    // pub password_hasher: Arc<PasswordHasher>,
    // pub password_validator: Arc<PasswordValidator>,

    /// 应用服务
    pub auth_app: Arc<dyn AuthApp + Send + Sync>,
    pub user_app: Arc<dyn UserApp + Send + Sync>,
}

impl AppState {
    pub async fn new(config: AppConfig) -> Result<Self, Error> {
        // // 初始化数据库
        // let db_pool = create_pg_pool(&config.database)
        //     .await
        //     .expect("Failed to connect to DB");
        // info!("Database connection established");
        // // 初始化redis
        // let redis_pool = create_redis_pool(&config.redis)
        //     .await
        //     .map_err(|e| anyhow!("Failed to connect to Redis: {}", e))?;
        //
        // // 运行数据库迁移
        // // sqlx::migrate!("./migrations").run(&db_pool).await?;
        // Ok(Self {
        //     db: db_pool,
        //     redis_pool,
        //     config,
        // })
        todo!()
    }
}
//
// #[derive(Clone)]
// pub struct App {
//     pub state: Arc<AppState>,
//     pub user_service: Arc<UserService>,
//     // TODO
//     // role_service: Arc<RoleService>,
// }
//
// impl App {
//     pub async fn new(config: Config) -> Result<Self, Error> {
//         let state = Arc::new(AppState::new(config).await?);
//         Ok(Self {
//             state: state.clone(),
//             user_service: Arc::new(UserService::new(state.clone())),
//         })
//     }
//
//     pub fn grpc_user_service(&self) -> GrpcUserService {
//         GrpcUserService::new(self.user_service.clone())
//     }
//     pub fn grpc_hello_service(&self) -> GrpcHelloService {
//         GrpcHelloService::default()
//     }
// }
