// use crate::config::Config;
// use crate::db::postgres::create_pg_pool;
// use crate::db::redis::create_redis_pool;
// use crate::grpc::hello::GrpcHelloService;
// use crate::grpc::user::GrpcUserService;
// use crate::services::user_service::UserService;
use crate::app::bootstrap::server::jwt::JwtService;
use crate::app::config::config::AppConfig;
use crate::application::services::auth_app::AuthApp;
use crate::application::services::user_app::UserApp;
use anyhow::Error;
use chrono::{DateTime, NaiveDate, Utc};
use dashmap::DashMap;
use deadpool_redis::Pool as RedisPool;
use lapin::{Channel, Connection as RabbitmqConnection};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use std::{
    collections::HashMap,
    sync::{
        Arc,
        atomic::{AtomicU16, AtomicU64},
    },
    u64,
};
use uuid::Uuid;
use crate::app::setup::AppServices;

#[derive(Clone)]
pub struct AppState {
    /// 配置
    pub services: Arc<AppServices>,
    /// 数据库池
    pub db_pool: PgPool,
    /// Redis连接池
    pub redis_pool: Option<RedisPool>,

    /// RabbitMQ连接
    // pub rabbitmq_connection: Option<Arc<RabbitmqConnection>>,
    // pub rabbitmq_channel: Option<Arc<Channel>>,

    /// 安全服务
    pub jwt_service: Arc<JwtService>,
    // pub password_hasher: Arc<PasswordHasher>,
    // pub password_validator: Arc<PasswordValidator>,
    /// 应用服务
    pub auth_app: Arc<dyn AuthApp + Send + Sync>,
    pub user_app: Arc<dyn UserApp + Send + Sync>,

    // pub request_stats: Arc<RequestStats>,
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

    // /// 记录请求统计
    // pub fn record_request(&self, endpoint: &str, method: &str, duration_ms: u64, success: bool) {
    //     // 更新总请求数
    //     self.request_stats
    //         .total_requests
    //         .fetch_add(1, std::sync::atomic::Ordering::SeqCst);
    //
    //     if success {
    //         self.request_stats
    //             .successful_requests
    //             .fetch_add(1, std::sync::atomic::Ordering::SeqCst);
    //     } else {
    //         self.request_stats
    //             .failed_requests
    //             .fetch_add(1, std::sync::atomic::Ordering::SeqCst);
    //     }
    //
    //     // 更新统计端点
    //     let endpoint_key = format!("{}:{}", method, endpoint);
    //     let mut entry = self
    //         .request_stats
    //         .endponit_stats
    //         .entry(endpoint_key.clone())
    //         .or_insert_with(|| EndpointStats {
    //             endpoint: endpoint.to_string(),
    //             method: method.to_string(),
    //             call_count: 0,
    //             success_count: 0,
    //             error_count: 0,
    //             avg_response_time_ms: 0.0,
    //             min_response_time_ms: u64::MAX,
    //             max_response_time_ms: 0,
    //             last_called_at: None,
    //         });
    //
    //     entry.call_count += 1;
    //     if success {
    //         entry.success_count += 1;
    //     } else {
    //         entry.error_count += 1;
    //     }
    //
    //     // 更新响应时间统计
    //     if duration_ms < entry.min_response_time_ms {
    //         entry.min_response_time_ms = duration_ms;
    //     }
    //     if duration_ms > entry.max_response_time_ms {
    //         entry.max_response_time_ms = duration_ms;
    //     }
    //
    //     // 计算平均响应时间
    //     let total_response_time = entry.avg_response_time_ms * (entry.call_count - 1) as f64;
    //     entry.avg_response_time_ms =
    //         (total_response_time + duration_ms as f64) / entry.call_count as f64;
    //
    //     entry.last_called_at = Some(Utc::now());
    // }
    //
    // pub fn record_user_request(&self, user_id: &Uuid, endpoint: &str) {
    //     let user_key = user_id.to_string();
    //     let mut entry = self
    //         .request_stats
    //         .user_stats
    //         .entry(user_key.clone())
    //         .or_insert_with(|| UserRequestStats {
    //             user_id: *user_id,
    //             total_requests: 0,
    //             last_request_at: None,
    //             active_days: Vec::new(),
    //             endpoint_distribution: HashMap::new(),
    //         });
    //     entry.total_requests += 1;
    //     entry.last_request_at = Some(Utc::now());
    //
    //     // 记录活跃日期
    //     let today = Utc::now().date_naive();
    //     if !entry.active_days.contains(&today) {
    //         entry.active_days.push(today);
    //     }
    //
    //     // 记录端点分布
    //     *entry
    //         .endpoint_distribution
    //         .entry(endpoint.to_string())
    //         .or_insert(0) += 1;
    // }
}

/// 请求统计
#[derive(Clone)]
pub struct RequestStats {
    /// 总请求数
    pub total_requests: Arc<AtomicU64>,
    /// 成功请求数
    pub successful_requests: Arc<AtomicU64>,
    /// 失败请求数
    pub failed_requests: Arc<AtomicU64>,
    /// 平均响应时间
    pub avg_response_time: Arc<AtomicU64>,
    /// 接口调用统计
    pub endponit_stats: Arc<DashMap<String, EndpointStats>>,
    /// 用户请求统计
    pub user_stats: Arc<DashMap<String, UserRequestStats>>,
}

/// 端点统计
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EndpointStats {
    /// 端点路径
    pub endpoint: String,
    /// HTTP方法
    pub method: String,
    /// 调用次数
    pub call_count: u64,
    /// 成功次数
    pub success_count: u64,
    /// 失败次数
    pub error_count: u64,
    /// 平均响应时间(毫秒)
    pub avg_response_time_ms: f64,
    /// 最小响应时间(毫秒)
    pub min_response_time_ms: u64,
    /// 最大响应时间(毫秒)
    pub max_response_time_ms: u64,
    /// 最后调用时间
    pub last_called_at: Option<DateTime<Utc>>,
}

/// 用户请求统计
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserRequestStats {
    /// 用户ID
    pub user_id: Uuid,
    /// 请求总数
    pub total_requests: u64,
    /// 最后请求时间
    pub last_request_at: Option<DateTime<Utc>>,
    /// 活跃天数
    pub active_days: Vec<NaiveDate>,
    /// 接口调用分布
    pub endpoint_distribution: HashMap<String, u64>,
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
// impls App {
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
