use crate::app::bootstrap::server::jwt::JwtService;
use crate::domain::shared::auth::user::AuthUser;
use common::security::jwt::claim::JwtClaim;
use std::sync::Arc;
use tonic::metadata::MetadataMap;
use tonic::service::Interceptor;
use tonic::{Code, Request, Status};
use tracing::{debug, error, info};

/// gRPC 上下文，包含认证用户信息
#[derive(Clone, Debug)]
pub struct GrpcContext {
    /// 认证用户
    pub auth_user: Option<AuthUser>,
    /// JWT 声明
    pub claims: Option<JwtClaim>,
    /// 客户端 IP 地址
    pub client_ip: Option<String>,
    /// 请求 ID
    pub request_id: Option<String>,
}

impl GrpcContext {
    /// 创建新的 gRPC 上下文
    pub fn new() -> Self {
        Self {
            auth_user: None,
            claims: None,
            client_ip: None,
            request_id: None,
        }
    }

    /// 从 JWT 声明创建上下文
    pub fn from_claims(claims: JwtClaim, client_ip: Option<String>) -> Self {
        let auth_user = Some(AuthUser::from_claims(&claims));
        Self {
            auth_user,
            claims: Some(claims),
            client_ip,
            request_id: None,
        }
    }

    /// 检查是否已认证
    pub fn is_authenticated(&self) -> bool {
        self.auth_user
            .as_ref()
            .map(|u| u.is_authenticated())
            .unwrap_or(false)
    }
}

impl Default for GrpcContext {
    fn default() -> Self {
        Self::new()
    }
}

/// 从 Metadata 中提取 token
pub fn extract_token_from_metadata(metadata: &MetadataMap) -> Option<String> {
    // 尝试从 Authorization header 提取
    metadata
        .get("authorization")
        .or_else(|| metadata.get("Authorization"))
        .and_then(|v| v.to_str().ok())
        .and_then(|v| {
            if v.starts_with("Bearer ") {
                Some(v[7..].to_string())
            } else {
                None
            }
        })
}

/// 从 Metadata 中提取客户端 IP
pub fn extract_client_ip(metadata: &MetadataMap) -> Option<String> {
    metadata
        .get("x-forwarded-for")
        .or_else(|| metadata.get("X-Forwarded-For"))
        .or_else(|| metadata.get("x-real-ip"))
        .or_else(|| metadata.get("X-Real-IP"))
        .and_then(|v| v.to_str().ok())
        .map(|s| s.split(',').next().unwrap_or(s).trim().to_string())
}

/// 从 Metadata 中提取请求 ID
pub fn extract_request_id(metadata: &MetadataMap) -> Option<String> {
    metadata
        .get("x-request-id")
        .or_else(|| metadata.get("X-Request-ID"))
        .and_then(|v| v.to_str().ok())
        .map(|s| s.to_string())
}

/// 认证拦截器
#[derive(Clone)]
pub struct AuthInterceptor {
    jwt_service: Arc<JwtService>,
}

impl AuthInterceptor {
    pub fn new(jwt_service: Arc<JwtService>) -> Self {
        Self { jwt_service }
    }
}

impl Interceptor for AuthInterceptor {
    fn call(&mut self, mut request: Request<()>) -> Result<Request<()>, Status> {
        // 提取 token
        let token = match request.metadata().get("authorization") {
            Some(auth_value) => {
                let auth_str = auth_value
                    .to_str()
                    .map_err(|_| Status::new(Code::Internal, "Invalid authorization header"))?;
                if let Some(token) = auth_str.strip_prefix("Bearer ") {
                    token.to_string()
                } else {
                    return Err(Status::new(
                        Code::Unauthenticated,
                        "Invalid authorization format",
                    ));
                }
            }
            None => {
                return Err(Status::new(
                    Code::Unauthenticated,
                    "Missing authorization header",
                ));
            }
        };

        // 验证 token
        let claims = self.jwt_service.verify_token_sync(&token).map_err(|e| {
            error!("JWT verification failed: {}", e);
            Status::new(Code::Unauthenticated, "Invalid or expired token")
        });

        // 提取客户端信息
        let client_ip = extract_client_ip(request.metadata());
        let request_id = extract_request_id(request.metadata());

        // 创建 gRPC 上下文
        let mut context = GrpcContext::from_claims(claims?, client_ip);
        context.request_id = request_id;

        // 将上下文添加到请求扩展中
        request.extensions_mut().insert(context);

        debug!("gRPC request authenticated successfully");
        Ok(request)
    }
}

/// 日志拦截器
#[derive(Clone)]
pub struct LoggingInterceptor;

impl LoggingInterceptor {
    pub fn new() -> Self {
        Self
    }
}

impl Interceptor for LoggingInterceptor {
    fn call(&mut self, request: Request<()>) -> Result<Request<()>, Status> {
        let start = std::time::Instant::now();

        // 提取请求信息
        let method = request
            .metadata()
            .get("x-grpc-method")
            .and_then(|v| v.to_str().ok())
            .unwrap_or("unknown")
            .to_string();

        let client_ip = extract_client_ip(request.metadata());

        info!(
            "gRPC request started: method={}, client_ip={:?}",
            method, client_ip
        );

        // 注意: 由于 Interceptor 是同步的，我们不能在此时记录耗时
        // 耗时应该在 ServerInterceptor 中记录

        Ok(request)
    }
}

/// 组合拦截器 - 用于同时使用认证和日志
pub fn create_interceptor_chain(
    jwt_service: Arc<JwtService>,
) -> impl Fn(Request<()>) -> Result<Request<()>, Status> + Clone {
    let auth = AuthInterceptor::new(jwt_service);
    move |mut request| {
        // 这里可以添加多个拦截器的链式调用
        // 目前先做简单的认证
        let mut auth_interceptor = auth.clone();
        auth_interceptor.call(request)
    }
}
