use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct ApiResponse<T> {
    pub code: i32,
    pub success: bool,
    pub data: Option<T>,
    pub error: Option<String>,
    pub trace_id: Option<String>,
    pub request_id: Option<String>,
    pub timestamp: Option<String>,
}


