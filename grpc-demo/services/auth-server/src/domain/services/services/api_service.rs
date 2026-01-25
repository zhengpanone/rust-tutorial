// src/domain/services/services/api_service.rs
use super::super::{
    models::{api::Api, module::Module},
    repositories::{ApiRepository, ModuleRepository, Pagination},
    value_objects::{ApiMethod, ApiStatus},
};

pub struct ApiDomainService<MR: ModuleRepository, AR: ApiRepository> {
    module_repository: MR,
    api_repository: AR,
}

impl<MR: ModuleRepository, AR: ApiRepository> ApiDomainService<MR, AR> {
    pub fn new(module_repository: MR, api_repository: AR) -> Self {
        Self {
            module_repository,
            api_repository,
        }
    }

    pub async fn create_api(
        &self,
        module_id: &str,
        api_path: String,
        api_name: String,
        method: ApiMethod,
        description: Option<String>,
        request_example: Option<String>,
        response_example: Option<String>,
    ) -> Result<Api, crate::shared::error::Error> {
        // 获取模块
        let module = self.get_module(module_id).await?;

        // 检查API路径和方法是否已存在
        let exists = self
            .api_repository
            .exists_by_path(&module.id, method, &api_path)
            .await?;

        if exists {
            return Err(crate::shared::error::Error::Duplicate(format!(
                "API路径 {} 已存在",
                api_path
            )));
        }

        // 创建API
        let api = Api::new(
            &module,
            api_path,
            api_name,
            method,
            description,
            request_example,
            response_example,
        )?;

        // 保存API
        let saved_api = self.api_repository.save(api).await?;

        // 更新模块的API计数
        let mut module = module;
        module.increment_apis_count();
        self.module_repository.save(module).await?;

        Ok(saved_api)
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

    pub async fn get_api(&self, api_id: &str) -> Result<Api, crate::shared::error::Error> {
        let id = super::super::models::api::ApiId::parse(api_id)?;
        self.api_repository
            .find_by_id(&id)
            .await?
            .ok_or_else(|| crate::shared::error::Error::NotFound(format!("API {} 不存在", api_id)))
    }

    pub async fn publish_api(&self, api_id: &str) -> Result<Api, crate::shared::error::Error> {
        let mut api = self.get_api(api_id).await?;
        api.publish();
        self.api_repository.save(api).await
    }

    pub async fn unpublish_api(&self, api_id: &str) -> Result<Api, crate::shared::error::Error> {
        let mut api = self.get_api(api_id).await?;
        api.unpublish();
        self.api_repository.save(api).await
    }

    pub async fn deprecate_api(&self, api_id: &str) -> Result<Api, crate::shared::error::Error> {
        let mut api = self.get_api(api_id).await?;
        api.deprecate();
        self.api_repository.save(api).await
    }

    pub async fn delete_api(&self, api_id: &str) -> Result<bool, crate::shared::error::Error> {
        // 获取API
        let api = self.get_api(api_id).await?;

        // 删除API
        let deleted = self.api_repository.delete(&api.id).await?;

        if deleted {
            // 获取模块并更新API计数
            let mut module = self.get_module(api.module_id.as_str()).await?;
            module.decrement_apis_count();
            self.module_repository.save(module).await?;
        }

        Ok(deleted)
    }

    pub async fn list_apis_by_module(
        &self,
        module_id: &str,
        pagination: Pagination,
    ) -> Result<(Vec<Api>, i64), crate::shared::error::Error> {
        let module_id_obj = super::super::models::module::ModuleId::parse(module_id)?;
        self.api_repository
            .find_by_module_id(&module_id_obj, &pagination)
            .await
    }

    pub async fn find_published_apis(&self) -> Result<Vec<Api>, crate::shared::error::Error> {
        self.api_repository
            .find_by_status(ApiStatus::Published)
            .await
    }
}
