// src/domain/services/events/service_events.rs
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use super::super::models::service::ServiceId;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceCreated {
    pub service_id: ServiceId,
    pub service_code: String,
    pub service_name: String,
    pub occurred_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceUpdated {
    pub service_id: ServiceId,
    pub service_name: String,
    pub occurred_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceDisabled {
    pub service_id: ServiceId,
    pub occurred_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceEnabled {
    pub service_id: ServiceId,
    pub occurred_at: DateTime<Utc>,
}
