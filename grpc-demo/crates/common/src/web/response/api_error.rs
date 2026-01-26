use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

// TODO
#[derive(Debug, Clone, Deserialize, Serialize, ToSchema)]
pub struct ApiError {
    code: i32,
    message: String,
}
