// src/domain/services/repository/service_repository
use super::super::{
    models::service::{Service, ServiceId},
    value_objects::{ServiceStatus, ServiceType},
};
use async_trait::async_trait;
use common::error::AppResult;

#[async_trait]
pub trait ServiceRepository: Send + Sync {
    async fn save(&self, service: Service) -> AppResult<Service>;

    async fn find_by_id(&self, id: &ServiceId) -> AppResult<Option<Service>>;

    async fn find_by_code(&self, code: &str) -> AppResult<Option<Service>>;
    async fn find_all(
        &self,
        filters: &ServiceFilters,
        pagination: &Pagination,
    ) -> AppResult<(Vec<Service>, i64)>;

    async fn delete(&self, id: &ServiceId) -> AppResult<bool>;
    async fn exists_by_code(&self, code: &str) -> AppResult<bool>;

    async fn count_by_status(&self, status: ServiceStatus) -> AppResult<i64>;
}

#[derive(Debug, Clone, Default)]
pub struct ServiceFilters {
    pub code: Option<String>,
    pub name: Option<String>,
    pub service_type: Option<ServiceType>,
    pub status: Option<ServiceStatus>,
    pub is_internal: Option<bool>,
}

#[derive(Debug, Clone)]
pub struct Pagination {
    pub page: i64,
    pub page_size: i64,
    pub offset: i64,
}

impl Pagination {
    pub fn new(page: Option<i64>, page_size: Option<i64>) -> Self {
        let page = page.unwrap_or(1).max(1);
        let page_size = page_size.unwrap_or(10).max(1).min(100);
        let offset = (page - 1) * page_size;

        Self {
            page,
            page_size,
            offset,
        }
    }
}
