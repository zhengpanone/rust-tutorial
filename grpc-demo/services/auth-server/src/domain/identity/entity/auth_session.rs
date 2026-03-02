use chrono::{DateTime, Utc};
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct AuthSession {
    pub session_id: String,
    pub user_id: Uuid,
    pub access_token: String,
    pub refresh_token: String,
    pub access_token_expires_at: DateTime<Utc>,
    pub refresh_token_expires_at: DateTime<Utc>,
    pub user_agent: Option<String>,
    pub ip_address: Option<String>,
    pub is_revoked: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl AuthSession {
    pub fn new(
        user_id: Uuid,
        access_token: String,
        refresh_token: String,
        access_token_expires_at: DateTime<Utc>,
        refresh_token_expires_at: DateTime<Utc>,
        user_agent: Option<String>,
        ip_address: Option<String>,
    ) -> Self {
        let now = Utc::now();

        Self {
            session_id: Uuid::new_v4().to_string(),
            user_id,
            access_token,
            refresh_token,
            access_token_expires_at,
            refresh_token_expires_at,
            user_agent,
            ip_address,
            is_revoked: false,
            created_at: now,
            updated_at: now,
        }
    }

    pub fn revoke(&mut self) {
        self.is_revoked = true;
        self.updated_at = Utc::now();
    }

    pub fn is_access_token_expired(&self) -> bool {
        Utc::now() > self.access_token_expires_at
    }

    pub fn is_refresh_token_expired(&self) -> bool {
        Utc::now() > self.refresh_token_expires_at
    }

    pub fn is_valid(&self) -> bool {
        !self.is_revoked && !self.is_access_token_expired()
    }

    pub fn refresh(
        &mut self,
        new_access_token: String,
        new_refresh_token: String,
        access_token_expires_at: DateTime<Utc>,
        refresh_token_expires_at: DateTime<Utc>,
    ) {
        self.access_token = new_access_token;
        self.refresh_token = new_refresh_token;
        self.access_token_expires_at = access_token_expires_at;
        self.refresh_token_expires_at = refresh_token_expires_at;
        self.updated_at = Utc::now();
    }
}
