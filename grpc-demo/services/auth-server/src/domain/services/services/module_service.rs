// src/domain/services/services/module_service.rs
use super::super::{
    events::{ModuleCreated, ModuleUpdated},
    models::{module::Module, service::Service},
    repositories::{ModuleRepository, Pagination, ServiceRepository},
};

pub struct ModuleDomainService<SR: ServiceRepository, MR: ModuleRepository> {
    service_repository: SR,
    module_repository: MR,
}

impl<SR: ServiceRepository, MR: ModuleRepository> ModuleDomainService<SR, MR> {
    pub fn new(service_repository: SR, module_repository: MR) -> Self {
        Self {
            service_repository,
            module_repository,
        }
    }

    pub async fn create_module(
        &self,
        service_id: &str,
        module_code: String,
        module_name: String,
        description: Option<String>,
    ) -> Result<(Module, Vec<ModuleCreated>), crate::shared::error::Error> {
        // 获取服务
        let service = self.get_service(service_id).await?;

        // 检查模块编码是否已存在
        let exists = self
            .module_repository
            .exists_by_code(&service.id, &module_code)
            .await?;

        if exists {
            return Err(crate::shared::error::Error::Duplicate(format!(
                "模块编码 {} 已存在",
                module_code
            )));
        }

        // 创建模块
        let (module, events) = Module::new(&service, module_code, module_name, description)?;

        // 保存模块
        let saved_module = self.module_repository.save(module).await?;

        // 更新服务的模块计数
        let mut service = service;
        service.increment_modules_count();
        self.service_repository.save(service).await?;

        Ok((saved_module, events))
    }

    pub async fn get_service(
        &self,
        service_id: &str,
    ) -> Result<Service, crate::shared::error::Error> {
        let id = super::super::models::service::ServiceId::parse(service_id)?;
        self.service_repository
            .find_by_id(&id)
            .await?
            .ok_or_else(|| {
                crate::shared::error::Error::NotFound(format!("服务 {} 不存在", service_id))
            })
    }

    pub async fn get_module(&self, module_id: &str) -> Result<Module, crate::shared::error::Error> {
        let id = super::super::models::module::ModuleId::parse(module_id)?;
        self.module_repository
            .find_by_id(&id)
            .await?
            .ok_or_else(|| {
                crate::shared::error::Error::NotFound(format!("模块 {} 不存在", module_id))
            })
    }

    pub async fn update_module(
        &self,
        module_id: &str,
        module_name: Option<String>,
        description: Option<String>,
    ) -> Result<(Module, Vec<ModuleUpdated>), crate::shared::error::Error> {
        let mut module = self.get_module(module_id).await?;

        // 更新模块信息
        module.update_info(module_name, description)?;

        // 创建更新事件
        let event = ModuleUpdated {
            module_id: module.id.clone(),
            module_name: module.module_name.clone(),
            occurred_at: chrono::Utc::now(),
        };

        // 保存更新
        let updated_module = self.module_repository.save(module).await?;

        Ok((updated_module, vec![event]))
    }

    pub async fn delete_module(
        &self,
        module_id: &str,
    ) -> Result<bool, crate::shared::error::Error> {
        // 获取模块
        let module = self.get_module(module_id).await?;

        // 删除模块
        let deleted = self.module_repository.delete(&module.id).await?;

        if deleted {
            // 获取服务并更新模块计数
            let mut service = self.get_service(module.service_id.as_str()).await?;
            service.decrement_modules_count();
            self.service_repository.save(service).await?;
        }

        Ok(deleted)
    }

    pub async fn list_modules_by_service(
        &self,
        service_id: &str,
        pagination: Pagination,
    ) -> Result<(Vec<Module>, i64), crate::shared::error::Error> {
        let service_id_obj = super::super::models::service::ServiceId::parse(service_id)?;
        self.module_repository
            .find_by_service_id(&service_id_obj, &pagination)
            .await
    }
}
