use crate::domain::identity::repositories::user_repository::UserRepository;
use std::sync::Arc;

/// 认证应用服务实现
#[derive(Clone)]
pub struct AuthAppImpl {
    user_repository: Arc<dyn UserRepository + Send + Sync>, // TODO
}
