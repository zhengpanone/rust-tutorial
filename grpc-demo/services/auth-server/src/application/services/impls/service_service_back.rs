use async_trait::async_trait;
use common::web::pagination::PaginatedResponse;
use std::sync::Arc;

use crate::{
    models::service::Service,
    repository::service_repository::{ServiceRepository, ServiceRepositoryTrait},
    dto::service_schemas::{CreateServiceDTO, UpdateServiceDTO},
    state::AppState,
};

#[derive(Debug)]
pub enum ServiceError {
    ValidationError(String),
    NotFound(String),
    DuplicateServiceCode(String),
    DatabaseError(String),
}

impl std::fmt::Display for ServiceError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ServiceError::ValidationError(msg) => write!(f, "Validation error: {}", msg),
            ServiceError::NotFound(msg) => write!(f, "Not found: {}", msg),
            ServiceError::DuplicateServiceCode(msg) => write!(f, "Duplicate service code: {}", msg),
            ServiceError::DatabaseError(msg) => write!(f, "Database error: {}", msg),
        }
    }
}

impl std::error::Error for ServiceError {}

#[async_trait]
pub trait ServiceServiceTrait: Send + Sync {
    async fn create_service(&self, request: CreateServiceDTO) -> Result<Service, ServiceError>;
    async fn get_service(&self, id: &str) -> Result<Service, ServiceError>;
    async fn list_services(
        &self,
        query: ServiceQuery,
    ) -> Result<PaginatedResponse<Service>, ServiceError>;
    async fn update_service(
        &self,
        id: &str,
        request: UpdateServiceDTO,
    ) -> Result<Service, ServiceError>;
    async fn delete_service(&self, id: &str) -> Result<(), ServiceError>;
}

#[derive(Clone)]
pub struct ServiceService {
    state: Arc<AppState>,
    repository: Box<dyn ServiceRepositoryTrait>,
}

impl ServiceService {
    pub fn new(state: Arc<AppState>) -> Self {
        let repository = ServiceRepository::new(state.db.clone());
        Self {
            state,
            repository: repository,
        }
    }
}

#[async_trait]
impl ServiceServiceTrait for ServiceService {
    async fn create_service(&self, request: CreateServiceDTO) -> Result<Service, ServiceError> {
        // 验证请求参数
        request
            .validate()
            .map_err(|e| ServiceError::ValidationError(format!("{:?}", e)))?;

        // 检查服务编码是否已存在
        let exists = self
            .repository
            .exists_by_service_code(&request.service_code)
            .await
            .map_err(|e| ServiceError::DatabaseError(e.to_string()))?;

        if exists {
            return Err(ServiceError::DuplicateServiceCode(format!(
                "服务编码 {} 已存在",
                request.service_code
            )));
        }

        // 创建服务
        let create_service = CreateService {
            service_code: request.service_code,
            service_name: request.service_name,
            service_type: request.service_type,
            service_desc: request.service_desc,
            owner_team: request.owner_team,
            base_url: request.base_url,
            status: request.status,
            health_endpoint: request.health_endpoint,
            is_internal: request.is_internal,
        };

        self.repository
            .create(&create_service)
            .await
            .map_err(|e| ServiceError::DatabaseError(e.to_string()))
    }

    async fn get_service(&self, id: &str) -> Result<Service, ServiceError> {
        self.repository
            .find_by_id(id)
            .await
            .map_err(|e| ServiceError::DatabaseError(e.to_string()))?
            .ok_or_else(|| ServiceError::NotFound(format!("服务 {} 不存在", id)))
    }

    async fn list_services(
        &self,
        query: ServiceQuery,
    ) -> Result<PaginatedResponse<Service>, ServiceError> {
        let pagination = PaginationParams::new(query.page, query.page_size);

        let (services, total) = self
            .repository
            .find_all(&query, &pagination)
            .await
            .map_err(|e| ServiceError::DatabaseError(e.to_string()))?;

        let total_pages = (total as f64 / pagination.page_size as f64).ceil() as i64;

        Ok(PaginatedResponse {
            items: services,
            total,
            page: pagination.page,
            page_size: pagination.page_size,
            total_pages,
        })
    }

    async fn update_service(
        &self,
        id: &str,
        request: UpdateServiceDTO,
    ) -> Result<Service, ServiceError> {
        // 验证请求参数
        request
            .validate()
            .map_err(|e| ServiceError::ValidationError(format!("{:?}", e)))?;

        // 检查服务是否存在
        let existing = self
            .repository
            .find_by_id(id)
            .await
            .map_err(|e| ServiceError::DatabaseError(e.to_string()))?;

        if existing.is_none() {
            return Err(ServiceError::NotFound(format!("服务 {} 不存在", id)));
        }

        // 更新服务
        let update_service = UpdateService {
            service_name: request.service_name,
            service_type: request.service_type,
            service_desc: request.service_desc,
            owner_team: request.owner_team,
            base_url: request.base_url,
            status: request.status,
            health_endpoint: request.health_endpoint,
            is_internal: request.is_internal,
        };

        self.repository
            .update(id, &update_service)
            .await
            .map_err(|e| ServiceError::DatabaseError(e.to_string()))?
            .ok_or_else(|| ServiceError::NotFound(format!("服务 {} 不存在", id)))
    }

    async fn delete_service(&self, id: &str) -> Result<(), ServiceError> {
        let deleted = self
            .repository
            .delete(id)
            .await
            .map_err(|e| ServiceError::DatabaseError(e.to_string()))?;

        if deleted {
            Ok(())
        } else {
            Err(ServiceError::NotFound(format!("服务 {} 不存在", id)))
        }
    }
}
