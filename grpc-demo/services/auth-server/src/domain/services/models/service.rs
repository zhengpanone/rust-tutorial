// src/domain/services/entities/service.rs
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

use super::super::{
    events::service_events::ServiceCreated,
    value_objects::{ServiceStatus, ServiceType, Url},
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ServiceId(String);

impl ServiceId {
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

impl From<String> for ServiceId {
    fn from(id: String) -> Self {
        Self(id)
    }
}

impl From<ServiceId> for String {
    fn from(id: ServiceId) -> Self {
        id.0
    }
}

impl std::fmt::Display for ServiceId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug, Clone)]
pub struct Service {
    pub id: ServiceId,
    pub service_code: String,
    pub service_name: String,
    pub service_type: ServiceType,
    pub description: Option<String>,
    pub owner_team: Option<String>,
    pub base_url: Option<Url>,
    pub status: ServiceStatus,
    pub health_endpoint: Option<String>,
    pub is_internal: bool,
    pub modules_count: u32,
    pub apis_count: u32,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl Service {
    pub fn new(
        service_code: String,
        service_name: String,
        service_type: ServiceType,
        description: Option<String>,
        owner_team: Option<String>,
        base_url: Option<Url>,
        status: ServiceStatus,
        health_endpoint: Option<String>,
        is_internal: bool,
    ) -> Result<(Self, Vec<ServiceCreated>), crate::shared::error::Error> {
        // 验证业务规则
        if service_code.is_empty() || service_code.len() > 50 {
            return Err(crate::shared::error::Error::Validation(
                "服务编码长度必须在1-50之间".to_string(),
            ));
        }

        if service_name.is_empty() || service_name.len() > 100 {
            return Err(crate::shared::error::Error::Validation(
                "服务名称长度必须在1-100之间".to_string(),
            ));
        }

        if let Some(ref desc) = description {
            if desc.len() > 500 {
                return Err(crate::shared::error::Error::Validation(
                    "服务描述长度不能超过500".to_string(),
                ));
            }
        }

        if let Some(ref team) = owner_team {
            if team.len() > 100 {
                return Err(crate::shared::error::Error::Validation(
                    "负责团队长度不能超过100".to_string(),
                ));
            }
        }

        if let Some(ref endpoint) = health_endpoint {
            if endpoint.len() > 200 {
                return Err(crate::shared::error::Error::Validation(
                    "健康检查端点长度不能超过200".to_string(),
                ));
            }
        }

        let now = Utc::now();
        let service = Self {
            id: ServiceId::new(),
            service_code,
            service_name,
            service_type,
            description,
            owner_team,
            base_url,
            status,
            health_endpoint,
            is_internal,
            modules_count: 0,
            apis_count: 0,
            created_at: now,
            updated_at: now,
        };

        // 创建领域事件
        let event = ServiceCreated {
            service_id: service.id.clone(),
            service_code: service.service_code.clone(),
            service_name: service.service_name.clone(),
            occurred_at: now,
        };

        Ok((service, vec![event]))
    }

    pub fn enable(&mut self) {
        self.status = ServiceStatus::Enabled;
        self.updated_at = Utc::now();
    }

    pub fn disable(&mut self) {
        self.status = ServiceStatus::Disabled;
        self.updated_at = Utc::now();
    }

    pub fn put_maintenance(&mut self) {
        self.status = ServiceStatus::Maintenance;
        self.updated_at = Utc::now();
    }

    pub fn update_info(
        &mut self,
        service_name: Option<String>,
        description: Option<String>,
        owner_team: Option<String>,
        base_url: Option<Url>,
        health_endpoint: Option<String>,
    ) -> Result<(), crate::shared::error::Error> {
        if let Some(name) = service_name {
            if name.is_empty() || name.len() > 100 {
                return Err(crate::shared::error::Error::Validation(
                    "服务名称长度必须在1-100之间".to_string(),
                ));
            }
            self.service_name = name;
        }

        if let Some(desc) = description {
            if desc.len() > 500 {
                return Err(crate::shared::error::Error::Validation(
                    "服务描述长度不能超过500".to_string(),
                ));
            }
            self.description = Some(desc);
        } else {
            self.description = None;
        }

        if let Some(team) = owner_team {
            if team.len() > 100 {
                return Err(crate::shared::error::Error::Validation(
                    "负责团队长度不能超过100".to_string(),
                ));
            }
            self.owner_team = Some(team);
        } else {
            self.owner_team = None;
        }

        if let Some(url) = base_url {
            self.base_url = Some(url);
        } else {
            self.base_url = None;
        }

        if let Some(endpoint) = health_endpoint {
            if endpoint.len() > 200 {
                return Err(crate::shared::error::Error::Validation(
                    "健康检查端点长度不能超过200".to_string(),
                ));
            }
            self.health_endpoint = Some(endpoint);
        } else {
            self.health_endpoint = None;
        }

        self.updated_at = Utc::now();
        Ok(())
    }

    pub fn increment_modules_count(&mut self) {
        self.modules_count += 1;
        self.updated_at = Utc::now();
    }

    pub fn decrement_modules_count(&mut self) {
        if self.modules_count > 0 {
            self.modules_count -= 1;
            self.updated_at = Utc::now();
        }
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
