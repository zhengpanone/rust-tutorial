use crate::app::bootstrap::InfrastructureServices;
use crate::app::config::config::AppConfig;
use crate::app::state::AppState;
use crate::application::services::auth_service::AuthService;
use crate::application::services::impls::auth_service_impl::AuthServiceImpl;
use crate::application::services::impls::user_service_impl::UserServiceImpl;

use common::error::AppResult;
use std::sync::Arc;

#[derive(Clone)]
pub struct AppServices {
    // 用户相关服务
    // pub user_service: Arc<dyn UserService>,
    pub auth_service: Arc<dyn AuthService>,
    // pub profile_service: Arc<ProfileService>,
    //
    // // 业务服务
    // pub order_service: Arc<OrderService>,
    // pub payment_service: Arc<PaymentService>,
    // pub inventory_service: Arc<InventoryService>,
    // pub shipping_service: Arc<ShippingService>,
    //
    // // 系统服务
    // pub notification_service: Arc<NotificationService>,
    // pub report_service: Arc<ReportService>,
    // pub audit_service: Arc<AuditService>,
    //
    // // 管理服务
    // pub admin_service: Arc<AdminService>,
    // pub system_service: Arc<SystemService>,
    // pub config_service: Arc<ConfigService>,
    //
    // // 事件处理器
    // pub event_handlers: HashMap<String, Vec<Arc<dyn EventHandler>>>,
    //
    // // 领域服务工厂
    // pub domain_service_factory: Arc<dyn DomainServiceFactory>,
    //
    // // 查询服务
    // pub query_services: HashMap<String, Arc<dyn QueryService>>,
}

impl AppServices {
    pub async fn new(
        _config: &AppConfig,
        _app_state: Arc<AppState>,
        infrastructure_services: Arc<InfrastructureServices>,
    ) -> AppResult<Self> {
        // 1. 获取基础设施适配器
        let user_repository = infrastructure_services.get_user_repository();
        // 2. 创建领域服务工厂

        // 3. 初始化应用服务
        let _user_service = Arc::new(UserServiceImpl::new(user_repository.clone()));
        let auth_service = Arc::new(AuthServiceImpl::new(
            user_repository.clone(),
            infrastructure_services.redis_pool.clone(),
        ));
        Ok(Self {
            // user_service,
            auth_service,
        })
    }
}
