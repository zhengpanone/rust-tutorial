use chrono::{DateTime, Duration, Utc};
use serde::{Deserialize, Serialize};
use sqlx::types::uuid;
use std::collections::HashMap;
use utoipa::ToSchema;
use validator::Validate;

/// JWT令牌类型
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, ToSchema)]
pub enum TokenType {
    #[serde(rename = "access")]
    Access,
    #[serde(rename = "refresh")]
    Refresh,
    #[serde(rename = "api_key")]
    ApiKey,
}

impl std::fmt::Display for TokenType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TokenType::Access => write!(f, "access"),
            TokenType::Refresh => write!(f, "refresh"),
            TokenType::ApiKey => write!(f, "api_key"),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Validate, ToSchema)]
pub struct JwtClaim {
    /// 令牌ID
    pub jti: String,

    /// 主题（用户ID）
    pub sub: String,

    /// 签发者
    pub iss: String,

    /// 受众
    pub aud: String,

    /// 签发时间
    pub iat: i64,
    /// 过期时间
    pub exp: i64,
    /// 生效时间
    pub nbf: i64,
    /// 令牌类型
    pub typ: TokenType,
    /// 用户信息
    pub user: JwtUser,

    /// 会话信息
    pub session: JwtSession,
    /// 自定义信息
    #[serde(skip_serializing_if = "HashMap::is_empty")]
    pub custom: HashMap<String, serde_json::Value>,
}

/// JWT 用户信息
#[derive(Debug, Clone, Serialize, Deserialize, Validate, ToSchema)]
pub struct JwtUser {
    /// 用户ID
    pub id: String,
    /// 用户名
    pub username: String,
    /// 用户邮箱
    pub email: String,
    /// 用户显示名
    pub display_name: String,
    /// 用户角色
    pub roles: Vec<String>,
    /// 权限列表
    pub permissions: Vec<String>,
    /// 用户状态
    pub status: UserStatus,
    /// 用户邮箱是否已验证
    pub email_verified: bool,
    /// 用户手机号是否已验证
    pub phone_verified: bool,
    /// 用户元信息
    #[serde(skip_serializing_if = "HashMap::is_empty")]
    pub metadata: HashMap<String, serde_json::Value>,
}

/// 用户状态
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, ToSchema)]
pub enum UserStatus {
    #[serde(rename = "active")]
    Active,
    #[serde(rename = "inactive")]
    Inactive,
    #[serde(rename = "suspended")]
    Suspended,
    #[serde(rename = "locked")]
    Locked,
    #[serde(rename = "deleted")]
    Deleted,
}

impl std::fmt::Display for UserStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            UserStatus::Active => write!(f, "active"),
            UserStatus::Inactive => write!(f, "inactive"),
            UserStatus::Suspended => write!(f, "suspended"),
            UserStatus::Locked => write!(f, "locked"),
            UserStatus::Deleted => write!(f, "deleted"),
        }
    }
}
/// JWT 会话信息
#[derive(Debug, Clone, Serialize, Deserialize, Validate, ToSchema)]
pub struct JwtSession {
    /// 会话ID
    pub id: String,
    /// 设备ID
    pub device_id: Option<String>,
    /// 设备类型
    pub device_type: Option<String>,
    /// 用户代理
    pub user_agent: Option<String>,
    /// IP地址
    pub ip_addr: Option<String>,
    /// 地理位置
    pub location: Option<String>,
    /// 是否首次登录
    pub is_first_login: bool,
    /// 登录时间
    pub login_time: DateTime<Utc>,
    /// 最后活动时间
    pub last_activity: DateTime<Utc>,
    /// 会话元信息
    #[serde(skip_serializing_if = "HashMap::is_empty")]
    pub metadata: HashMap<String, serde_json::Value>,
}

impl JwtClaim {
    // 创建新的访问令牌
    pub fn new_access_token(
        user_id: String,
        user: JwtUser,
        session: JwtSession,
        expires_in_minutes: i64,
    ) -> Self {
        let now = Utc::now();
        let iat = now.timestamp();
        let jti = uuid::Uuid::new_v4().to_string();

        let exp = now + Duration::minutes(expires_in_minutes);
        let nbf = iat;
        let iss = "https://example.com".to_string();
        let aud = "https://example.com".to_string();
        let typ = TokenType::Access;
        Self {
            jti,
            sub: user_id,
            iss,
            aud,
            iat,
            exp: exp.timestamp(),
            nbf,
            typ,
            user,
            session,
            custom: HashMap::new(),
        }
    }
    /// 创建新的刷新令牌
    pub fn new_refresh_token(
        user_id: String,
        session: JwtSession,
        expires_in_minutes: i64,
    ) -> Self {
        let now = Utc::now();
        let iat = now.timestamp();
        let jti = uuid::Uuid::new_v4().to_string();
        let exp = now + Duration::minutes(expires_in_minutes);
        Self {
            jti,
            sub: user_id.clone(),
            iss: "https://example.com".to_string(),
            aud: "https://example.com".to_string(),
            iat,
            exp: exp.timestamp(),
            nbf: iat,
            typ: TokenType::Refresh,
            user: JwtUser {
                id: user_id.clone(),
                username: "".to_string(),
                email: "".to_string(),
                roles: vec![],
                display_name: "".to_string(),
                permissions: vec![],
                status: UserStatus::Active,
                email_verified: false,
                phone_verified: false,
                metadata: HashMap::new(),
            },
            session,
            custom: HashMap::new(),
        }
    }
    /// 检查令牌是否已过期
    pub fn is_expired(&self) -> bool {
        let now = Utc::now().timestamp();
        now > self.exp
    }

    /// 检查令牌是否有效
    pub fn is_valid(&self) -> bool {
        let now = Utc::now().timestamp();
        now >= self.nbf && now <= self.exp
    }
    /// 检查令牌类型
    pub fn is_access_token(&self) -> bool {
        self.typ == TokenType::Access
    }
    pub fn is_refresh_token(&self) -> bool {
        self.typ == TokenType::Refresh
    }
    /// 获取剩余有效期（秒）
    pub fn expires_in(&self) -> Duration {
        Duration::seconds(self.exp - self.iat)
    }
    /// 添加自定义信息
    pub fn with_custom(
        mut self,
        key: impl Into<String>,
        value: impl Into<serde_json::Value>,
    ) -> Self {
        self.custom.insert(key.into(), value.into());
        self
    }

    pub fn validate(&self) -> Result<(), JwtValidationError> {
        // 检查令牌是否已过期
        if self.is_expired() {
            return Err(JwtValidationError::Expired);
        }
        let now = Utc::now().timestamp();
        if now < self.nbf {
            return Err(JwtValidationError::NotYetValid);
        }
        if self.jti.is_empty() {
            return Err(JwtValidationError::MissingJti);
        }
        if self.sub.is_empty() {
            return Err(JwtValidationError::MissingSub);
        }
        Ok(())
    }
}

#[derive(Debug, thiserror::Error)]
pub enum JwtValidationError {
    /// 令牌已过期
    #[error("Token is expired")]
    Expired,
    /// 令牌尚未生效
    #[error("Token is not yet valid")]
    NotYetValid,
    /// 缺少令牌ID
    #[error("Missing JTI")]
    MissingJti,
    /// 缺少主题
    #[error("Missing Sub")]
    MissingSub,
    /// 令牌类型错误
    #[error("Invalid Token Type:{0}")]
    InvalidTokenTyp(String),
    /// 签名验证失败
    #[error("Invalid Signature")]
    InvalidSignature,
    /// 算法错误
    #[error("Invalid Algorithm")]
    InvalidAlgorithm,
    /// 无效的令牌格式
    #[error("Invalid Token Format")]
    InvalidFormat,
    /// 令牌被吊销了
    #[error("Token is Revoked")]
    Revoked,
    /// 未知错误
    #[error("Unknown error: {0}")]
    UnknownError(String),
}
