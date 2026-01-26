// src/app/middleware/http/auth.rs
use crate::app::state::AppState;
use axum::extract::{Request, State};
use axum::http::HeaderMap;
use axum::middleware::Next;
use axum::response::Response;
use common::error::AppError;
use common::web::response::ApiResponse;

pub async fn jwt_auth_middleware(
    State(state): State<std::sync::Arc<AppState>>,
    headers: HeaderMap,
    mut request: Request,
    next: Next,
) -> Result<Response, ApiResponse<()>> {
    // 从请求头中获取JWT令牌
    let token = extract_token_from_headers(&headers).ok_or_else(|| {
        ApiResponse::from_app_error(AppError::Authentication("缺少JWT令牌".to_string()))
    })?;
    // 验证JWT令牌
    // let claims = state.jwt_service.verify_token(&token)?;

    // if let Some(token) = token {}
    // next.into_inner(request)
    todo!()
}

/// 从请求头中提取JWT令牌
fn extract_token_from_headers(headers: &HeaderMap) -> Option<String> {
    // 从Authorization头中提取JWT令牌

    if let Some(auth_header) = headers.get("Authorization") {
        if let Ok(auth_str) = auth_header.to_str() {
            if auth_str.starts_with("Bearer ") {
                let token = &auth_str[7..];
                return Some(token.to_string());
            }
        }

        // 从X-Access-Token头中提取JWT令牌
    }
    if let Some(access_header) = headers.get("X-Access-Token") {
        if let Ok(access_str) = access_header.to_str() {
            return Some(access_str.to_string());
        }
    }
    None
}
