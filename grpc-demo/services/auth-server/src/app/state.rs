use crate::app::config::config::AppConfig;

use anyhow::Error;
use chrono::{DateTime, NaiveDate, Utc};
use dashmap::DashMap;

use serde::{Deserialize, Serialize};

use crate::app::bootstrap::InfrastructureServices;
use crate::application::services::auth_service::AuthService;
use crate::application::services::impls::auth_service_impl::AuthServiceImpl;
use crate::application::services::impls::user_service_impl::UserServiceImpl;
use crate::application::services::user_service::UserService;
use common::error::AppResult;
use std::{
    collections::HashMap,
    sync::{
        Arc,
        atomic::{AtomicU16, AtomicU64},
    },
    u64,
};
use tracing::info;
use uuid::Uuid;

/// 应用状态
#[derive(Clone)]
pub struct AppState {
    /// 配置
    pub config: AppConfig,
    // 基础设施服务
    pub infrastructure_services: Arc<InfrastructureServices>,

    pub user_service: Arc<dyn UserService>,
    pub auth_service: Arc<dyn AuthService>,
    pub startup_time: DateTime<Utc>,
}

pub async fn init_app_state(
    config: &AppConfig,
    infrastructure_services: InfrastructureServices,
) -> AppResult<Arc<AppState>> {
    info!("🎯 Initializing application state...");

    let state = Arc::new(AppState::new(config).await?);

    Ok(state)
}

impl AppState {
    pub async fn new(config: &AppConfig) -> AppResult<Self> {
        // 1. 获取基础设施适配器
        let infrastructure_services = InfrastructureServices::new(config).await?;
        let user_repository = infrastructure_services.get_user_repository();
        // 2. 创建领域服务工厂

        // 3. 初始化应用服务
        let user_service = Arc::new(UserServiceImpl::new(user_repository.clone()));
        let auth_service = Arc::new(AuthServiceImpl::new(
            user_repository.clone(),
            infrastructure_services.redis_pool.clone(),
        ));
        Ok(Self {
            config: config.clone(),
            infrastructure_services: Arc::new(infrastructure_services),
            user_service,
            auth_service,
            startup_time: Utc::now(),
        })
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
