use crate::domain::services::models::service::{Service, ServiceId};
use crate::domain::services::repository::service_repository::{
    Pagination, ServiceFilters, ServiceRepository,
};
use crate::domain::services::value_objects::ServiceStatus;
use async_trait::async_trait;
use common::error::AppResult;
use sqlx::PgPool;
use std::sync::Arc;

/// 服务仓储实现
pub struct ServiceRepositoryImpl {
    pool: PgPool,
    redis_pool: Option<Arc<deadpool_redis::Pool>>,
}
impl ServiceRepositoryImpl {
    pub fn new(pool: PgPool, redis_pool: Option<Arc<deadpool_redis::Pool>>) -> Self {
        Self { pool, redis_pool }
    }
}
#[async_trait]
impl ServiceRepository for ServiceRepositoryImpl {
    async fn save(&self, service: Service) -> AppResult<Service> {
        todo!()
    }

    async fn find_by_id(&self, id: &ServiceId) -> AppResult<Option<Service>> {
        todo!()
    }

    async fn find_by_code(&self, code: &str) -> AppResult<Option<Service>> {
        todo!()
    }

    async fn find_all(
        &self,
        filters: &ServiceFilters,
        pagination: &Pagination,
    ) -> AppResult<(Vec<Service>, i64)> {
        todo!()
    }

    async fn delete(&self, id: &ServiceId) -> AppResult<bool> {
        todo!()
    }

    async fn exists_by_code(&self, code: &str) -> AppResult<bool> {
        todo!()
    }

    async fn count_by_status(&self, status: ServiceStatus) -> AppResult<i64> {
        todo!()
    }
}
