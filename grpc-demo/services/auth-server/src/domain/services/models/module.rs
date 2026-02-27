// src/domain/services/models/module.rs
use chrono::{DateTime, Utc};
use uuid::Uuid;

use super::super::events::module_events::ModuleCreated;
use super::{Service, ServiceId};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ModuleId(String);

impl ModuleId {
    pub fn new() -> Self {
        Self(Uuid::new_v4().to_string())
    }

    pub fn parse(id: &str) -> Result<Self, crate::shared::error::Error> {
        Uuid::parse_str(id)
            .map_err(|_| crate::shared::error::Error::InvalidId(id.to_string()))
            .map(|_| Self(id.to_string()))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl From<String> for ModuleId {
    fn from(id: String) -> Self {
        Self(id)
    }
}

impl From<ModuleId> for String {
    fn from(id: ModuleId) -> Self {
        id.0
    }
}

impl std::fmt::Display for ModuleId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug, Clone)]
pub struct Module {
    pub id: ModuleId,
    pub service_id: ServiceId,
    pub module_code: String,
    pub module_name: String,
    pub description: Option<String>,
    pub apis_count: u32,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl Module {
    pub fn new(
        service: &Service,
        module_code: String,
        module_name: String,
        description: Option<String>,
    ) -> Result<(Self, Vec<ModuleCreated>), crate::shared::error::Error> {
        // 验证业务规则
        if module_code.is_empty() || module_code.len() > 50 {
            return Err(crate::shared::error::Error::Validation(
                "模块编码长度必须在1-50之间".to_string(),
            ));
        }

        if module_name.is_empty() || module_name.len() > 100 {
            return Err(crate::shared::error::Error::Validation(
                "模块名称长度必须在1-100之间".to_string(),
            ));
        }

        if let Some(ref desc) = description {
            if desc.len() > 500 {
                return Err(crate::shared::error::Error::Validation(
                    "模块描述长度不能超过500".to_string(),
                ));
            }
        }

        let now = Utc::now();
        let module = Self {
            id: ModuleId::new(),
            service_id: service.id.clone(),
            module_code,
            module_name,
            description,
            apis_count: 0,
            created_at: now,
            updated_at: now,
        };

        // 创建领域事件
        let event = ModuleCreated {
            module_id: module.id.clone(),
            service_id: module.service_id.clone(),
            module_code: module.module_code.clone(),
            module_name: module.module_name.clone(),
            occurred_at: now,
        };

        Ok((module, vec![event]))
    }

    pub fn update_info(
        &mut self,
        module_name: Option<String>,
        description: Option<String>,
    ) -> Result<(), crate::shared::error::Error> {
        if let Some(name) = module_name {
            if name.is_empty() || name.len() > 100 {
                return Err(crate::shared::error::Error::Validation(
                    "模块名称长度必须在1-100之间".to_string(),
                ));
            }
            self.module_name = name;
        }

        if let Some(desc) = description {
            if desc.len() > 500 {
                return Err(crate::shared::error::Error::Validation(
                    "模块描述长度不能超过500".to_string(),
                ));
            }
            self.description = Some(desc);
        } else {
            self.description = None;
        }

        self.updated_at = Utc::now();
        Ok(())
    }

    pub fn increment_apis_count(&mut self) {
        self.apis_count += 1;
        self.updated_at = Utc::now();
    }

    pub fn decrement_apis_count(&mut self) {
        if self.apis_count > 0 {
            self.apis_count -= 1;
            self.updated_at = Utc::now();
        }
    }
}
