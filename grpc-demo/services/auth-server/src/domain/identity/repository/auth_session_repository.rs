use crate::domain::identity::models::auth_session::AuthSession;
use async_trait::async_trait;
use common::error::AppResult;

#[async_trait]
pub trait AuthSessionRepository: Send + Sync {
    async fn save(&self, session: AuthSession) -> AppResult<AuthSession>;
    async fn find_by_id(&self, session_id: &str) -> AppResult<Option<AuthSession>>;
    async fn find_by_access_token(&self, token: &str) -> AppResult<Option<AuthSession>>;
    async fn find_by_refresh_token(&self, token: &str) -> AppResult<Option<AuthSession>>;
    async fn find_by_user_id(&self, user_id: &str) -> AppResult<Vec<AuthSession>>;
    async fn revoke(&self, session_id: &str) -> AppResult<bool>;
    async fn revoke_all_user_sessions(&self, user_id: &str) -> AppResult<u64>;
    async fn cleanup_expired_sessions(&self) -> AppResult<u64>;
}
