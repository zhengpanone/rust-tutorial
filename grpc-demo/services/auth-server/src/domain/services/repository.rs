// src/domain/services/repository.rs
#[async_trait]
pub trait ServiceRepositoryTrait: Send + Sync {
    async fn create(&self, service: &Service) -> Result<Service, SqlxError>;
    async fn find_by_id(&self, id: &str) -> Result<Option<Service>, SqlxError>;
    async fn find_by_service_code(&self, service_code: &str) -> Result<Option<Service>, SqlxError>;
    async fn find_all(
        &self,
        query: &ServiceQuery,
        pagination: &PaginationParams,
    ) -> Result<(Vec<Service>, i64), SqlxError>;
    async fn update(
        &self,
        id: &str,
        update_service: &Service,
    ) -> Result<Option<Service>, SqlxError>;
    async fn delete(&self, id: &str) -> Result<bool, SqlxError>;
    async fn exists_by_service_code(&self, service_code: &str) -> Result<bool, SqlxError>;
}
