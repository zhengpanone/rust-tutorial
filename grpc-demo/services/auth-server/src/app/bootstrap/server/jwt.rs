use crate::app::bootstrap::InfrastructureServices;
use crate::app::config::config::SecurityConfig;
use chrono::Utc;
use common::error::AppError;
use common::security::jwt::claim::{JwtClaim, JwtSession, JwtUser, UserStatus};
use deadpool_redis::redis::AsyncCommands;
use jsonwebtoken::{Algorithm, DecodingKey, EncodingKey, Header, encode};
use parking_lot::RwLock;
use serde_json::json;
use std::fs;
use std::path::Path;
use std::sync::Arc;
use tracing::{info, warn};

pub async fn init_jwt_service(
    config: &SecurityConfig,
    services: &InfrastructureServices,
) -> Result<Arc<JwtService>, AppError> {
    info!("🛡️  Initializing jwt services...");

    let jwt_service = JwtService::from_config(config, services.redis_pool.clone())?;
    Ok(Arc::new(jwt_service))
}

fn generate_or_load_jwt_secret(config: &SecurityConfig) -> Result<String, AppError> {
    if config.jwt_secret.starts_with("file://") {
        let file_path = config.jwt_secret.trim_start_matches("file://");
        return load_jwt_secret_from_file(file_path);
    }
    info!("🔑 Using provided JWT secret: {}", config.jwt_secret);
    Ok(config.jwt_secret.clone())
}

/// 从文件中加载JWT密钥
#[allow(dead_code)]
fn load_jwt_secret_from_file(file_path: &str) -> Result<String, AppError> {
    info!("📁 Loading JWT secret from file: {}", file_path);
    let path = Path::new(file_path);

    if !path.exists() {
        // 文件不存在，生成新密钥
        warn!("JWT secret file not found, generating new secret");
        return generate_jwt_save_jwt_secret(file_path);
    }
    // 读取文件内容
    let secret_str = fs::read_to_string(path)
        .map_err(|e| AppError::Config(format!("Failed to read secret from file: {}", e)))?;

    let secret_str = secret_str.trim().to_string();
    if secret_str.is_empty() {
        warn!("JWT secret file is empty, generating new secret");
        AppError::Config("JWT secret file is empty".to_string());
    }
    if secret_str.len() < 32 {
        warn!(
            "JWT secret from file is too short ({} chars), should be at least 32 chars",
            secret_str.len()
        );
    }
    info!("✅ JWT secret loaded from file: {}", file_path);
    Ok(secret_str)
}

fn generate_jwt_save_jwt_secret(file_path: &str) -> Result<String, AppError> {
    use rand::RngCore;
    info!("🔐 Generating new JWT secret...");

    // 生成64字节的随机密钥
    let mut secret_bytes = [0u8; 64];
    rand::thread_rng().fill_bytes(&mut secret_bytes);

    // 转换为base64
    let secret_str = base64::encode(&secret_bytes);
    // 确保目录存在
    let path = Path::new(file_path);
    if let Some(parent) = path.parent() {
        fs::create_dir_all(&parent)
            .map_err(|e| AppError::Config(format!("Failed to create directory: {}", e)))?;
    }

    // 保存到文件
    fs::write(path, &secret_str)
        .map_err(|e| AppError::Config(format!("Failed to write secret to file: {}", e)))?;
    // 设置文件权限(Unix only)
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let mut perms = fs::metadata(path)
            .map_err(|e| AppError::Config(format!("Failed to get file metadata: {}", e)))?
            .permissions();
        perms.set_mode(0o600); // 只有所有者可读写
        fs::set_permissions(path, perms)?;
    }
    info!("✅ New JWT secret generated and saved to: {}", file_path);
    Ok(secret_str)
}
/// JWT服务配置
#[derive(Debug, Clone)]
pub struct JwtConfig {
    /// 访问令牌密钥
    pub access_token_secret: String,

    /// 刷新令牌密钥
    pub refresh_token_secret: String,

    /// 访问令牌过期时间（分钟）
    pub access_token_expiry_minutes: i64,

    /// 刷新令牌过期时间（天）
    pub refresh_token_expiry_days: i64,

    /// 是否启用令牌刷新
    pub enable_token_refresh: bool,

    /// 算法
    pub algorithm: Algorithm,

    /// 是否启用吊销列表
    pub enable_revocation_list: bool,

    /// 吊销列表TTL（秒）
    pub revocation_list_ttl_seconds: u64,
}

impl Default for JwtConfig {
    fn default() -> Self {
        Self {
            access_token_secret: "your-access-token-secret-change-in-production".to_string(),
            refresh_token_secret: "your-refresh-token-secret-change-in-production".to_string(),
            access_token_expiry_minutes: 15, // 15分钟
            refresh_token_expiry_days: 7,    // 7天
            enable_token_refresh: true,
            algorithm: Algorithm::HS256,
            enable_revocation_list: true,
            revocation_list_ttl_seconds: 86400, // 24小时
        }
    }
}

// JWT服务
#[derive(Clone)]
pub struct JwtService {
    config: Arc<JwtConfig>,
    redis_client: Option<Arc<deadpool_redis::Pool>>,
    encoding_key: Arc<RwLock<EncodingKey>>,
    _decoding_key: Arc<RwLock<DecodingKey>>,
}

impl JwtService {
    /// 创建JWT服务
    pub fn new(
        config: JwtConfig,
        redis_client: Option<Arc<deadpool_redis::Pool>>,
    ) -> Result<Self, AppError> {
        if config.access_token_secret.is_empty() {
            return Err(AppError::Config("Access token secret is empty".to_string()));
        }
        if config.refresh_token_secret.is_empty() {
            return Err(AppError::Config(
                "Refresh token secret is empty".to_string(),
            ));
        }
        if config.access_token_expiry_minutes <= 0 {
            return Err(AppError::Config(
                "Access token expiry minutes must be greater than 0".to_string(),
            ));
        }
        if config.refresh_token_expiry_days <= 0 {
            return Err(AppError::Config(
                "Refresh token expiry minutes must be greater than 0".to_string(),
            ));
        }
        // 生成编码密钥
        let encoding_key = match config.algorithm {
            Algorithm::HS256 | Algorithm::HS384 | Algorithm::HS512 => {
                EncodingKey::from_secret(config.access_token_secret.as_bytes())
            }
            Algorithm::RS256 | Algorithm::RS384 | Algorithm::RS512 => {
                EncodingKey::from_rsa_pem(config.access_token_secret.as_bytes())
                    .map_err(|e| AppError::Config(format!("Invalid RSA key:{}", e)))?
            }
            Algorithm::ES256 | Algorithm::ES384 => {
                EncodingKey::from_ec_pem(config.access_token_secret.as_bytes())
                    .map_err(|e| AppError::Config(format!("Invalid EC key:{}", e)))?
            }
            Algorithm::PS256 | Algorithm::PS384 | Algorithm::PS512 => {
                EncodingKey::from_rsa_pem(config.access_token_secret.as_bytes())
                    .map_err(|e| AppError::Config(format!("Invalid PS key:{}", e)))?
            }
            _ => return Err(AppError::Config("Unsupported algorithm".to_string())),
        };
        let decoding_key = match config.algorithm {
            Algorithm::HS256 | Algorithm::HS384 | Algorithm::HS512 => {
                DecodingKey::from_secret(config.access_token_secret.as_bytes())
            }
            Algorithm::RS256 | Algorithm::RS384 | Algorithm::RS512 => {
                DecodingKey::from_rsa_pem(config.access_token_secret.as_bytes())
                    .map_err(|e| AppError::Config(format!("Invalid RSA key:{}", e)))?
            }
            Algorithm::ES256 | Algorithm::ES384 => {
                DecodingKey::from_ec_pem(config.access_token_secret.as_bytes())
                    .map_err(|e| AppError::Config(format!("Invalid EC key:{}", e)))?
            }
            Algorithm::PS256 | Algorithm::PS384 | Algorithm::PS512 => {
                DecodingKey::from_rsa_pem(config.access_token_secret.as_bytes())
                    .map_err(|e| AppError::Config(format!("Invalid RSA key:{}", e)))?
            }
            _ => return Err(AppError::Config("Unsupported algorithm".to_string())),
        };
        info!(
            "JWT服务初始化完成, 算法: {:?},访问令牌过期时间: {}分钟, 刷新令牌过期时间: {}天",
            config.algorithm, config.access_token_expiry_minutes, config.refresh_token_expiry_days
        );
        Ok(Self {
            config: Arc::new(config),
            redis_client,
            encoding_key: Arc::new(RwLock::new(encoding_key)),
            _decoding_key: Arc::new(RwLock::new(decoding_key)),
        })
    }

    /// 从配置创建JWT服务
    pub fn from_config(
        config: &SecurityConfig,
        redis_client: Option<Arc<deadpool_redis::Pool>>,
    ) -> Result<Self, AppError> {
        let jwt_config = JwtConfig {
            access_token_secret: config.jwt_secret.clone(),
            refresh_token_secret: format!("{}_refresh", config.jwt_secret),
            access_token_expiry_minutes: config.jwt_expiry_minutes as i64,
            refresh_token_expiry_days: config.refresh_token_expiry_days as i64,
            ..Default::default()
        };
        Self::new(jwt_config, redis_client)
    }
    /// 生成访问令牌
    pub async fn generate_access_token(
        &self,
        user: JwtUser,
        session: JwtSession,
    ) -> Result<String, AppError> {
        let claim = JwtClaim::new_access_token(
            user.id.clone(),
            user,
            session,
            self.config.access_token_expiry_minutes,
        );
        // 验证声明
        claim
            .validate()
            .map_err(|e| AppError::Authentication(e.to_string()))?;
        // 创建头部
        let mut header = Header::new(self.config.algorithm);
        header.typ = Some("JWT".to_string());
        header.kid = Some("access".to_string());
        // 生成令牌
        let token = encode(&header, &claim, &self.encoding_key.read())
            .map_err(|e| AppError::Authentication(format!("生成访问令牌失败 {}", e)))?;
        info!("生成访问令牌成功: {}", claim.jti);
        Ok(token)
    }

    /// 生成刷新令牌
    pub async fn generate_refresh_token(
        &self,
        user_id: String,
        session: JwtSession,
    ) -> Result<(String, JwtClaim), AppError> {
        let claim =
            JwtClaim::new_refresh_token(user_id, session, self.config.refresh_token_expiry_days);
        // 验证声明
        claim
            .validate()
            .map_err(|e| AppError::Authentication(e.to_string()))?;
        // 创建头部
        let mut header = Header::new(self.config.algorithm);
        header.typ = Some("JWT".to_string());
        header.kid = Some("refresh".to_string());
        // 生成令牌
        let token = encode(&header, &claim, &self.encoding_key.read())
            .map_err(|e| AppError::Authentication(format!("生成刷新令牌失败 {}", e)))?;

        // 存储刷新令牌
        if self.config.enable_token_refresh {
            self.store_refresh_token(
                claim.jti.clone(),
                claim.sub.clone(),
                claim.session.id.to_string(),
            )
            .await?;
        }

        info!("生成刷新令牌成功: {}", claim.jti.clone());
        Ok((token, claim))
    }

    /// 存储刷新令牌
    async fn store_refresh_token(
        &self,
        token_id: String,
        user_id: String,
        session_id: String,
    ) -> Result<(), AppError> {
        if let Some(redis_client) = &self.redis_client {
            use redis::AsyncCommands;
            // 获取连接
            let mut conn = redis_client
                .get()
                .await
                .map_err(|e| AppError::Cache(format!("获取Redis连接失败: {}", e)))?;

            let key = format!("jwt:refresh:{}", token_id);
            let value = json!({
                "user_id": user_id,
                "session_id": session_id,
                "created_at": Utc::now().timestamp(),
                "used": false,
            });

            let _: () = conn
                .set_ex(
                    &key,
                    serde_json::to_string(&value).unwrap(),
                    (self.config.refresh_token_expiry_days * 24 * 60 * 60) as u64,
                )
                .await
                .map_err(|e| AppError::Cache(format!("存储刷新令牌失败：{}", e)))?;
        }
        Ok(())
    }

    /// 检查刷新令牌是否有效
    #[allow(dead_code)]
    async fn is_refresh_token_valid(&self, token_id: String) -> Result<bool, AppError> {
        if let Some(redis_client) = &self.redis_client {
            let mut conn = redis_client
                .get()
                .await
                .map_err(|e| AppError::Cache(format!("获取Redis连接失败: {}", e)))?;

            let key = format!("jwt:refresh:{}", token_id);

            let data: Option<String> = conn
                .get(&key)
                .await
                .map_err(|e| AppError::Cache(format!("获取刷新令牌失败：{}", e)))?;
            if let Some(data_str) = data {
                let data: serde_json::Value = serde_json::from_str(&data_str)
                    .map_err(|e| AppError::Serialization(format!("解析刷新令牌失败：{}", e)))?;
                if let Some(used) = data.get("used").and_then(|v| v.as_bool()) {
                    return Ok(!used);
                }
            }
        }
        Ok(false)
    }
    /// 标记刷新令牌已使用
    async fn mark_refresh_token_used(&self, token_id: String) -> Result<(), AppError> {
        if let Some(redis_client) = &self.redis_client {
            let mut conn = redis_client
                .get()
                .await
                .map_err(|e| AppError::Cache(format!("获取Redis连接失败: {}", e)))?;

            let key = format!("jwt:refresh:{}", token_id);

            let data: Option<String> = conn
                .get(&key)
                .await
                .map_err(|e| AppError::Cache(format!("获取刷新令牌失败：{}", e)))?;
            if let Some(data_str) = data {
                let mut data: serde_json::Value = serde_json::from_str(&data_str)
                    .map_err(|e| AppError::Serialization(format!("解析刷新令牌失败：{}", e)))?;

                data["used"] = serde_json::Value::Bool(true);

                let _: () = conn
                    .set(&key, serde_json::to_string(&data).unwrap())
                    .await
                    .map_err(|e| AppError::Cache(format!("标记刷新令牌已使用失败：{}", e)))?;
            }
        }
        Ok(())
    }
    /// 检查令牌是否已吊销
    async fn is_token_revoked(&self, token_id: String) -> Result<bool, AppError> {
        if let Some(redis_client) = &self.redis_client {
            let mut conn = redis_client
                .get()
                .await
                .map_err(|e| AppError::Cache(format!("获取Redis连接失败: {}", e)))?;

            let key = format!("jwt:revoked:{}", token_id);

            let exists: Option<bool> = conn
                .exists(&key)
                .await
                .map_err(|e| AppError::Cache(format!("检查吊销令牌失败：{}", e)))?;
            if let Some(exists) = exists {
                return Ok(exists);
            }
        }
        Ok(false)
    }
    /// 获取用户信息用于刷新令牌
    async fn get_user_for_refresh(&self, _token_id: String) -> Result<JwtUser, AppError> {
        // TODO 从数据库中获取用户信息
        Ok(JwtUser {
            id: "1".to_string(),
            username: "admin".to_string(),
            email: "<EMAIL>".to_string(),
            display_name: "".to_string(),
            roles: vec!["admin".to_string()],
            permissions: vec![],
            status: UserStatus::Activate,
            email_verified: false,
            phone_verified: false,
            metadata: Default::default(),
        })
    }

    /// 解析令牌，不验证
    pub fn parse_token_unverified(&self, token: String) -> Result<JwtClaim, AppError> {
        // 手动分割 JWT token
        let parts: Vec<&str> = token.split('.').collect();
        if parts.len() != 3 {
            return Err(AppError::Authentication("令牌格式错误".to_string()));
        }

        // 解析 payload 部分（中间部分）
        let payload_base64 = parts
            .get(1)
            .ok_or_else(|| AppError::Authentication("缺少载荷".to_string()))?;

        // Base64 URL 解码
        use base64::{Engine as _, engine::general_purpose::URL_SAFE_NO_PAD};
        let payload_bytes = URL_SAFE_NO_PAD
            .decode(payload_base64)
            .map_err(|e| AppError::Authentication(format!("解码载荷失败: {}", e)))?;

        // JSON 反序列化
        let claim: JwtClaim = serde_json::from_slice(&payload_bytes)
            .map_err(|e| AppError::Serialization(format!("反序列化令牌失败: {}", e)))?;
        Ok(claim)
    }
    /// 获取配置
    pub fn config(&self) -> &JwtConfig {
        &self.config
    }
}
