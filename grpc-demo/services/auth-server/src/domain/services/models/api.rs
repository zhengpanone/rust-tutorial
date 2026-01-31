// src/domain/services/entities/api.rs
use chrono::{DateTime, Utc};
use uuid::Uuid;
use validator::Validate;

use super::super::value_objects::{ApiMethod, ApiStatus, Url};
use super::{Module, ModuleId};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ApiId(String);

impl ApiId {
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

impl From<String> for ApiId {
    fn from(id: String) -> Self {
        Self(id)
    }
}

impl From<ApiId> for String {
    fn from(id: ApiId) -> Self {
        id.0
    }
}

impl std::fmt::Display for ApiId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug, Clone)]
pub struct Api {
    pub id: ApiId,
    pub module_id: ModuleId,
    pub api_path: String,
    pub api_name: String,
    pub method: ApiMethod,
    pub description: Option<String>,
    pub request_example: Option<String>,
    pub response_example: Option<String>,
    pub status: ApiStatus,
    pub deprecated_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl Api {
    pub fn new(
        module: &Module,
        api_path: String,
        api_name: String,
        method: ApiMethod,
        description: Option<String>,
        request_example: Option<String>,
        response_example: Option<String>,
    ) -> Result<Self, crate::shared::error::Error> {
        // 验证业务规则
        if api_path.is_empty() || api_path.len() > 200 {
            return Err(crate::shared::error::Error::Validation(
                "API路径长度必须在1-200之间".to_string(),
            ));
        }

        if api_name.is_empty() || api_name.len() > 100 {
            return Err(crate::shared::error::Error::Validation(
                "API名称长度必须在1-100之间".to_string(),
            ));
        }

        if let Some(ref desc) = description {
            if desc.len() > 500 {
                return Err(crate::shared::error::Error::Validation(
                    "API描述长度不能超过500".to_string(),
                ));
            }
        }

        if let Some(ref req) = request_example {
            if req.len() > 2000 {
                return Err(crate::shared::error::Error::Validation(
                    "请求示例长度不能超过2000".to_string(),
                ));
            }
        }

        if let Some(ref res) = response_example {
            if res.len() > 2000 {
                return Err(crate::shared::error::Error::Validation(
                    "响应示例长度不能超过2000".to_string(),
                ));
            }
        }

        let now = Utc::now();
        Ok(Self {
            id: ApiId::new(),
            module_id: module.id.clone(),
            api_path,
            api_name,
            method,
            description,
            request_example,
            response_example,
            status: ApiStatus::Draft,
            deprecated_at: None,
            created_at: now,
            updated_at: now,
        })
    }

    pub fn publish(&mut self) {
        self.status = ApiStatus::Published;
        self.updated_at = Utc::now();
    }

    pub fn unpublish(&mut self) {
        self.status = ApiStatus::Draft;
        self.updated_at = Utc::now();
    }

    pub fn deprecate(&mut self) {
        self.status = ApiStatus::Deprecated;
        self.deprecated_at = Some(Utc::now());
        self.updated_at = Utc::now();
    }

    pub fn update_info(
        &mut self,
        api_name: Option<String>,
        description: Option<String>,
        request_example: Option<String>,
        response_example: Option<String>,
    ) -> Result<(), crate::shared::error::Error> {
        if let Some(name) = api_name {
            if name.is_empty() || name.len() > 100 {
                return Err(crate::shared::error::Error::Validation(
                    "API名称长度必须在1-100之间".to_string(),
                ));
            }
            self.api_name = name;
        }

        if let Some(desc) = description {
            if desc.len() > 500 {
                return Err(crate::shared::error::Error::Validation(
                    "API描述长度不能超过500".to_string(),
                ));
            }
            self.description = Some(desc);
        } else {
            self.description = None;
        }

        if let Some(req) = request_example {
            if req.len() > 2000 {
                return Err(crate::shared::error::Error::Validation(
                    "请求示例长度不能超过2000".to_string(),
                ));
            }
            self.request_example = Some(req);
        } else {
            self.request_example = None;
        }

        if let Some(res) = response_example {
            if res.len() > 2000 {
                return Err(crate::shared::error::Error::Validation(
                    "响应示例长度不能超过2000".to_string(),
                ));
            }
            self.response_example = Some(res);
        } else {
            self.response_example = None;
        }

        self.updated_at = Utc::now();
        Ok(())
    }
}
