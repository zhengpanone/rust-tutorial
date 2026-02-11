use crate::app::config::config::AppConfig;
use crate::app::setup::AppServices;
use crate::app::state::{AppState, init_app_state};
use crate::domain::identity::repositories::user_repository::UserRepository;
use crate::infrastructure::persistence::repositories::user_repository_impl::UserRepositoryImpl;
use common::error::{AppError, AppResult};
use std::error::Error;
use std::sync::Arc;
use tracing::{error, info};

// 启动器主模块
pub mod database; // 数据库初始化
mod health; // 健康检查
mod logger; // 日志初始化
mod rabbitmq;
mod redis;
pub mod server;

/// 应用引导器
/// 1.初始化所有组件
/// 2. 配置依赖注入
/// 3.启动应用程序
/// 4. 管理生命周期
pub struct AppBootstrap {
    config: AppConfig,
    app_state: Option<Arc<AppState>>,
    infrastructure_services: Arc<InfrastructureServices>,
    app_services: Arc<AppServices>,
}

impl AppBootstrap {
    pub async fn new(config: AppConfig) -> Self {
        // 1. 初始化AppState
        let app_state = Arc::new(
            AppState::new(&config)
                .await
                .map_err(|e| AppError::Internal(e.to_string()))
                .unwrap(),
        );
        // 2. 初始化基础设施服务
        let infrastructure_services = Arc::new(InfrastructureServices::new(&config).await.unwrap());
        // 3. 初始化应用服务
        let app_services = Arc::new(
            AppServices::new(&config, app_state.clone(), infrastructure_services.clone())
                .await
                .unwrap(),
        );
        Self {
            config,
            app_state: Some(app_state),
            infrastructure_services,
            app_services,
        }
    }

    /// 完整启动流程
    pub async fn run(&mut self) -> AppResult<()> {
        info!("🚀 Starting application bootstrap...");
        // 1. 初始化日志
        let _guard = logger::init_logger(&self.config.logging);
        info!("🚀 Logger initialized");
        // 2. 显示启动横幅
        self.show_banner();
        // 3. 检查环境
        self.check_environment().expect("TODO: panic message");

        // 4. 初始化基础设施服务
        let infrastructure_services = self.init_infrastructure().await?;
        self.infrastructure_services = Arc::new(infrastructure_services.clone());

        // 5. 初始化应用服务
        let state = self
            .init_application_services(infrastructure_services.clone())
            .await?;
        self.app_state = Some(state.clone());

        // 6. 启动服务器
        self.start_server(state.clone()).await?;

        Ok(())
    }

    /// 显示启动横幅
    fn show_banner(&self) {
        info!("========================================");
        info!("       Microservice Manager v{}", env!("CARGO_PKG_VERSION"));
        info!("========================================");
        info!("Environment: {}", self.config.environment);
        info!(
            "HTTP Server: {}:{}",
            self.config.server.host, self.config.server.port
        );
        // info!(
        //     "gRPC Server: {}:{}",
        //     self.config.grpc.host, self.config.grpc.port
        // );
        info!("Debug Mode: {}", cfg!(debug_assertions));
        info!("========================================");
    }

    /// 检查环境配置
    fn check_environment(&self) -> Result<(), Box<dyn Error>> {
        if self.config.is_prod() {
            // 生产环境安全检查
            // if self.config.security.jwt_secret.contains("secret") {
            //     error!("⚠️  WARNING: Using default JWT secret in production!");
            // }

            if self.config.logging.log_level == "debug" {
                info!("⚠️  WARNING: Using debug logging in production");
            }
        }
        Ok(())
    }
    /// 初始化基础设施服务
    async fn init_infrastructure(&self) -> AppResult<InfrastructureServices> {
        info!("🔧 Initializing infrastructure...");
        let database_pool = database::init_database(&self.config.database).await?;
        let (redis_client, redis_pool) = redis::init_redis(&self.config.redis).await?;
        // let message_queue = rabbitmq::init_rabbitmq(&self.config.rabbitmq).await?;
        let services = InfrastructureServices {
            database_pool,
            redis_pool,
            // message_queue,
        };
        info!("🔧 Infrastructure initialized");
        Ok(services)
    }

    async fn init_application_services(
        &self,
        infra_services: InfrastructureServices,
    ) -> AppResult<Arc<AppState>> {
        info!("🎯 Initializing application services...");
        let app_state = init_app_state(&self.config, infra_services.clone()).await?;
        Ok(app_state)
    }
    async fn start_server(&self, state: Arc<AppState>) -> Result<(), AppError> {
        info!("🌐 Starting servers...");
        match (self.config.server.enable_http, self.config.grpc.enabled) {
            (true, true) => {
                // 启动混合服务器
                todo!()
            }
            (true, false) => {
                // 只启动HTTP服务器
                server::http::start_http_server(state.as_ref().clone(), &self.config.server)
                    .await?;
            }
            (false, true) => {
                // 只启动gRPC服务器
                todo!()
            }
            _ => {
                error!("❌ No server enabled. Enable at least HTTP or gRPC server");
                return Err(AppError::Internal(
                    "No server enabled. Enable at least HTTP or gRPC server".to_string(),
                ));
            }
        }
        Ok(())
    }
}

/// 基础设施服务集合
#[derive(Clone)]
pub struct InfrastructureServices {
    pub database_pool: sqlx::PgPool,
    pub redis_pool: Option<Arc<deadpool_redis::Pool>>,
    // pub search: Arc<dyn SearchEngine + Send + Sync>,

    // 消息队列
    // pub message_queue: Arc<dyn MessageQueue + Send + Sync>,
    // pub event_bus: Arc<dyn EventBus + Send + Sync>,
    // pub message_queue: Option<lapin::Connection>,
    //
    // // 外部API
    // pub http_client: Arc<dyn HttpClient + Send + Sync>,
    // pub api_clients: HashMap<String, Arc<dyn ApiClient + Send + Sync>>,
    //
    // // 文件存储
    // pub storage: Arc<dyn Storage + Send + Sync>,
    // pub cdn: Arc<dyn Cdn + Send + Sync>,
    //
    // // 认证授权
    // pub auth_service.proto: Arc<dyn AuthService + Send + Sync>,
    // pub authorization_service: Arc<dyn AuthorizationService + Send + Sync>,
    //
    // // 监控
    // pub metrics_client: Arc<dyn MetricsClient + Send + Sync>,
    // pub logging_client: Arc<dyn LoggingClient + Send + Sync>,
    // pub tracing_client: Arc<dyn TracingClient + Send + Sync>,
    //
    // // 其他
    // pub email_service: Arc<dyn EmailService + Send + Sync>,
    // pub sms_service: Arc<dyn SmsService + Send + Sync>,
    // pub notification_service: Arc<dyn NotificationService + Send + Sync>,
    //
    // // 配置
    // pub config_service: Arc<dyn ConfigService + Send + Sync>,
    //
    // // 任务调度
    // pub scheduler: Arc<dyn Scheduler + Send + Sync>,
    // pub job_queue: Arc<dyn JobQueue + Send + Sync>,
}

impl InfrastructureServices {
    pub async fn new(config: &AppConfig) -> AppResult<Self> {
        let database_pool = database::init_database(&config.database).await?;
        let (redis_client, redis_pool) = redis::init_redis(&config.redis).await?;

        // // 3. 初始化消息队列
        // let message_queue = Self::init_message_queue(config, app_state.clone()).await?;
        //
        // // 4. 初始化HTTP客户端
        // let http_client = Self::init_http_client(config, app_state.clone()).await?;
        //
        // // 5. 初始化认证服务
        // let auth_service.proto = Self::init_auth_service(config, app_state.clone()).await?;
        //
        // // 6. 初始化监控
        // let metrics_client = Self::init_metrics(config, app_state.clone()).await?;
        //
        // // 7. 初始化其他服务
        // let email_service = Self::init_email_service(config, app_state.clone()).await?;
        //
        // // 8. 初始化配置服务
        // let config_service = Self::init_config_service(config, app_state.clone()).await?;

        Ok(Self {
            database_pool,
            redis_pool,
        })
    }

    // 为领域层提供接口实现
    pub fn get_user_repository(&self) -> Arc<dyn UserRepository> {
        Arc::new(UserRepositoryImpl::new(
            self.database_pool.clone(),
            self.redis_pool.clone(),
        ))
    }
}
