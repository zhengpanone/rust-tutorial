use crate::api::http::configure_routes;
use crate::app::config::config::AppConfig;
use crate::app::state::AppState;
use axum::{middleware, Router};
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
mod services;

/// 服务器启动
pub struct AppBootstrap {
    config: Arc<AppConfig>,
    state: Option<Arc<AppState>>,
    services: Option<InfrastructureServices>,
}

impl AppBootstrap {
    pub fn new(config: AppConfig) -> Self {
        Self {
            config: Arc::new(config),
            state: None,
            services: None,
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
        self.services = Some(infrastructure_services.clone());

        // 初始化应用服务
        let app_services = self
            .init_application_services(&infrastructure_services)
            .await?;
        // 6. 启动服务器
        self.start_server(app_services.clone()).await?;

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
        info!(
            "gRPC Server: {}:{}",
            self.config.grpc.host, self.config.grpc.port
        );
        info!("Debug Mode: {}", cfg!(debug_assertions));
        info!("========================================");
    }

    /// 检查环境配置
    fn check_environment(&self) -> Result<(), Box<dyn Error>> {
        if self.config.is_prod() {
            // 生产环境安全检查
            if self.config.security.jwt_secret.contains("secret") {
                error!("⚠️  WARNING: Using default JWT secret in production!");
            }

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

    /// 初始化应用服务
    async fn init_application_services(
        &self,
        infra_services: &InfrastructureServices,
    ) -> AppResult<Arc<AppState>> {
        info!("🎯 Initializing application services...");
        let state = services::init_app_state(self.config.clone(), infra_services).await?;
        info!("🔧 Application services initialized");
        Ok(state)
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
                todo!()
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
        todo!()
    }

    fn configure_http_router(&self, state: Arc<AppState>) -> Router {
        // 1. 配置API路由
        let mut app = configure_routes();

        // 2. 添加状态
        app = app.with_state(state.clone());
        // 3. 添加全局中间件
        app = self.add_global_middleware(app);

        // 4. 添加OpenAPI文档
        app = self.add_openapi_docs(app);

        // 5. 添加监控端点
        app = self.add_monitoring_endpoints(app);
        app
    }

    fn add_global_middleware(self, app: Router<Arc<AppState>>) -> Router<Arc<AppState>> {
        use axum::middleware;
        app.layer(middleware::from_fn(request_logger))
    }

    pub fn state(&self) -> Option<&Arc<AppState>> {
        self.state.as_ref()
    }
}

/// 基础设施服务集合
#[derive(Clone)]
pub struct InfrastructureServices {
    pub database_pool: sqlx::PgPool,
    pub redis_pool: Option<Arc<deadpool_redis::Pool>>,
    // pub message_queue: Option<lapin::Connection>,
}
