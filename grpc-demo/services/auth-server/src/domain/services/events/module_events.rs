// src/domain/services/events/module_events.rs
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use super::super::models::{service::ServiceId, module::ModuleId};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModuleCreated {
    pub module_id: ModuleId,
    pub service_id: ServiceId,
    pub module_code: String,
    pub module_name: String,
    pub occurred_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModuleUpdated {
    pub module_id: ModuleId,
    pub module_name: String,
    pub occurred_at: DateTime<Utc>,
}