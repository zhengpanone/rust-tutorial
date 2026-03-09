// src/app/middleware/http/auth/extractor.rs
// 认证用户提取器

pub(crate) use crate::domain::shared::auth::user::AuthUser;
use axum::extract::FromRequestParts;
use axum::http::request::Parts;
use common::error::AppError;
use common::security::jwt::claim::JwtClaim;
use serde::Serialize;
use tracing::{debug, error};

impl<S> FromRequestParts<S> for AuthUser
where
    S: Send + Sync,
{
    type Rejection = AppError;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        debug!("🔍 Extracting AuthUser from request");

        // 从扩展中获取JWT声明
        let claims = parts.extensions.get::<JwtClaim>().ok_or_else(|| {
            error!("JWT claims not found in request extensions");
            AppError::Authentication("需要认证".to_string())
        })?;

        // 从扩展中获取用户信息
        if let Some(auth_user) = parts.extensions.get::<AuthUser>() {
            debug!(
                "AuthUser found in request extensions: {}",
                auth_user.user_id
            );
            return Ok(auth_user.clone());
        }
        // 从JWT声明中提取用户信息
        let auth_user = AuthUser::from_claims(claims);
        debug!("AuthUser found in JWT claims: {}", auth_user.user_id);
        Ok(auth_user)
    }
}
