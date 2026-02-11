// src/domain/services/services/service_service_back
use super::super::{
    events::{ServiceCreated, ServiceDisabled, ServiceEnabled, ServiceUpdated},
    models::service::Service,
    repositories::{Pagination, ServiceFilters, ServiceRepository},
    value_objects::{ServiceStatus, ServiceType, Url},
};

pub struct ServiceDomainService<R: ServiceRepository> {
    repository: R,
}

impl<R: ServiceRepository> ServiceDomainService<R> {
    pub fn new(repository: R) -> Self {
        Self { repository }
    }

    pub async fn create_service(
        &self,
        service_code: String,
        service_name: String,
        service_type: ServiceType,
        description: Option<String>,
        owner_team: Option<String>,
        base_url: Option<Url>,
        status: ServiceStatus,
        health_endpoint: Option<String>,
        is_internal: bool,
    ) -> Result<(Service, Vec<ServiceCreated>), crate::shared::error::Error> {
        // 检查服务编码是否已存在
        let exists = self.repository.exists_by_code(&service_code).await?;
        if exists {
            return Err(crate::shared::error::Error::Duplicate(format!(
                "服务编码 {} 已存在",
                service_code
            )));
        }

        // 创建服务
        let (service, events) = Service::new(
            service_code,
            service_name,
            service_type,
            description,
            owner_team,
            base_url,
            status,
            health_endpoint,
            is_internal,
        )?;

        // 保存到仓储
        let saved_service = self.repository.save(service).await?;

        Ok((saved_service, events))
    }

    pub async fn get_service(
        &self,
        service_id: &str,
    ) -> Result<Service, crate::shared::error::Error> {
        let id = super::super::models::service::ServiceId::parse(service_id)?;
        self.repository.find_by_id(&id).await?.ok_or_else(|| {
            crate::shared::error::Error::NotFound(format!("服务 {} 不存在", service_id))
        })
    }

    pub async fn update_service(
        &self,
        service_id: &str,
        service_name: Option<String>,
        description: Option<String>,
        owner_team: Option<String>,
        base_url: Option<Url>,
        health_endpoint: Option<String>,
    ) -> Result<(Service, Vec<ServiceUpdated>), crate::shared::error::Error> {
        let mut service = self.get_service(service_id).await?;

        // 更新服务信息
        service.update_info(
            service_name,
            description,
            owner_team,
            base_url,
            health_endpoint,
        )?;

        // 创建更新事件
        let event = ServiceUpdated {
            service_id: service.id.clone(),
            service_name: service.service_name.clone(),
            occurred_at: chrono::Utc::now(),
        };

        // 保存更新
        let updated_service = self.repository.save(service).await?;

        Ok((updated_service, vec![event]))
    }

    pub async fn enable_service(
        &self,
        service_id: &str,
    ) -> Result<(Service, Vec<ServiceEnabled>), crate::shared::error::Error> {
        let mut service = self.get_service(service_id).await?;
        service.enable();

        // 创建启用事件
        let event = ServiceEnabled {
            service_id: service.id.clone(),
            occurred_at: chrono::Utc::now(),
        };

        // 保存更新
        let enabled_service = self.repository.save(service).await?;

        Ok((enabled_service, vec![event]))
    }

    pub async fn disable_service(
        &self,
        service_id: &str,
    ) -> Result<(Service, Vec<ServiceDisabled>), crate::shared::error::Error> {
        let mut service = self.get_service(service_id).await?;
        service.disable();

        // 创建禁用事件
        let event = ServiceDisabled {
            service_id: service.id.clone(),
            occurred_at: chrono::Utc::now(),
        };

        // 保存更新
        let disabled_service = self.repository.save(service).await?;

        Ok((disabled_service, vec![event]))
    }

    pub async fn delete_service(
        &self,
        service_id: &str,
    ) -> Result<bool, crate::shared::error::Error> {
        let id = super::super::models::service::ServiceId::parse(service_id)?;
        self.repository.delete(&id).await
    }

    pub async fn list_services(
        &self,
        filters: ServiceFilters,
        pagination: Pagination,
    ) -> Result<(Vec<Service>, i64), crate::shared::error::Error> {
        self.repository.find_all(&filters, &pagination).await
    }
}
