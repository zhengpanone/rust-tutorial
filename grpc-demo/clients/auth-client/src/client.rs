use crate::error::UserClientError;
use crate::interceptor::client_interceptor::ClientInterceptor;
use proto::user::user_service_grpc_client::UserServiceGrpcClient;
use proto::user::{GetUserRequest, UserResponse};
use std::fmt::Debug;
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::Semaphore;
use tonic::codegen::InterceptedService;
use tonic::transport::{Channel, Endpoint};
use tracing::debug;
use tracing_attributes::instrument;

#[derive(Clone)]
pub struct UserClient {
    inner: UserServiceGrpcClient<InterceptedService<Channel, ClientInterceptor>>,
    max_concurrency: usize,
    semaphore: Arc<tokio::sync::Semaphore>,
    max_retry: usize,
}

impl UserClient {
    /// 通过地址创建（带拦截器 + 超时）
    ///
    /// ```rust
    /// let client = UserClient::connect("http://127.0.0.1:50051").await?;
    ///
    /// let endpoint = Endpoint::from_shared(dst.into())?.tcp_nodelay(true);
    /// let channel = endpoint.connect().await?;
    /// ```
    pub async fn connect<D>(dst: D) -> Result<Self, UserClientError>
    where
        D: Into<String>,
    {
        let endpoint = Endpoint::from_shared(dst.into())?
            .timeout(Duration::from_secs(3))
            .connect_timeout(Duration::from_secs(3))
            .tcp_keepalive(Some(Duration::from_secs(60)))
            .tcp_nodelay(true);

        let channel = endpoint.connect().await?;

        Ok(Self::from_channel(channel))
    }

    /// 从已有 Channel 构建（高级用法）
    pub fn from_channel(channel: Channel) -> Self {
        let interceptor = ClientInterceptor::new();
        let max_concurrency = 10; // 默认最大并发，可改为 Builder 参数
        let semaphore = Arc::new(tokio::sync::Semaphore::new(max_concurrency));
        let max_retry = 3; // 默认重试次数

        Self {
            inner: UserServiceGrpcClient::with_interceptor(channel, interceptor),
            max_concurrency,
            semaphore,
            max_retry,
        }
    }

    /// 获取用户（示例方法）
    ///  #[instrument(skip(self))] 是 tracing crate 提供的一个 宏属性
    /// tracing：核心库，提供 info!, debug!, span! 等宏
    /// tracing-attributes：提供宏属性 #[instrument]，可以直接加在函数上
    /// skip(self) 如果函数参数里有不想打印或不可打印的类型（比如 &self 或 &mut self），可以用 skip(...)
    #[instrument(skip(self))]
    pub async fn get_user(
        &mut self,
        id: impl Into<String> + Debug,
    ) -> Result<UserResponse, UserClientError> {
        let _permit = self.semaphore.acquire().await.unwrap(); // 并发限流
        let id = id.into();
        let mut attempt = 0;

        loop {
            attempt += 1;
            let request = GetUserRequest { id: id.clone() };
            // Tracing span
            // 用了 #[instrument]，就不需要再手动创建 span 了。
            // let span = tracing::info_span!("get_user", user_id = %request.id);
            // let _enter = span.enter();
            debug!(attempt, "尝试获取用户");
            // 记录到 span 的 field 中
            tracing::Span::current().record("user_id", &tracing::field::display(&id));

            let result = self.inner.clone().get_user(request).await;
            match result {
                Ok(resp) => {
                    // Metrics: 成功
                    let _ = metrics::counter!("user_client_requests_total",  "method" => "GetUser","status"=>"ok")
                        .increment(1);
                    tracing::info!("成功获取用户");
                    return Ok(resp.into_inner());
                }
                Err(err) => {
                    // Metrics: 失败
                    let _ = metrics::counter!("user_client_requests_total", "method" => "GetUser","status"=>"error")
                        .increment(1);
                    tracing::warn!(error = ?err, attempt, "获取用户失败");

                    if attempt >= self.max_retry {
                        tracing::error!("达到最大重试次数: {}", attempt);
                        return Err(err.into());
                    }
                    // Retry backoff
                    let backoff =
                        Duration::from_millis(50u64.saturating_mul(2u64.pow(attempt as u32)));
                    tracing::debug!(?backoff, "等待后重试");
                    tokio::time::sleep(backoff).await;
                }
            }
        }
    }
}

#[derive(Default)]
pub struct UserClientBuilder {
    endpoint: Option<String>,
    timeout: Option<Duration>,
    connect_timeout: Option<Duration>,
    tcp_keepalive: Option<Duration>,
    enable_tracing: bool,
    interceptor: Option<ClientInterceptor>,
    max_concurrency: Option<usize>,
    max_retry: Option<usize>,
}
impl UserClientBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn endpoint(mut self, endpoint: impl Into<String>) -> Self {
        self.endpoint = Some(endpoint.into());
        self
    }
    pub fn timeout(mut self, timeout: Duration) -> Self {
        self.timeout = Some(timeout);
        self
    }
    pub fn connect_timeout(mut self, timeout: Duration) -> Self {
        self.connect_timeout = Some(timeout);
        self
    }
    pub fn tcp_keepalive(mut self, duration: Duration) -> Self {
        self.tcp_keepalive = Some(duration);
        self
    }
    pub fn enable_tracing(mut self, enable: bool) -> Self {
        self.enable_tracing = enable;
        self
    }
    pub fn interceptor(mut self, interceptor: ClientInterceptor) -> Self {
        self.interceptor = Some(interceptor);
        self
    }
    pub fn max_concurrency(mut self, max: usize) -> Self {
        self.max_concurrency = Some(max);
        self
    }
    pub fn max_retry(mut self, retry: usize) -> Self {
        self.max_retry = Some(retry);
        self
    }

    pub async fn build(self) -> Result<UserClient, UserClientError> {
        let endpoint = self.endpoint.ok_or(UserClientError::MissingEndpoint)?;
        let mut ep = Endpoint::from_shared(endpoint)?;

        if let Some(t) = self.timeout {
            ep = ep.timeout(t)
        }
        if let Some(t) = self.connect_timeout {
            ep = ep.connect_timeout(t)
        }
        if let Some(t) = self.tcp_keepalive {
            ep = ep.tcp_keepalive(Some(t))
        }

        // 可选 Tracing
        if self.enable_tracing {
            tracing::info!("UserClient tracing enabled");
        }
        let max_concurrency = self.max_concurrency.unwrap_or(10);
        let semaphore = Arc::new(Semaphore::new(max_concurrency));
        let max_retry = self.max_retry.unwrap_or(3);
        // 构建 Channel
        let channel = ep.connect().await?;
        // 拦截器
        let interceptor = self.interceptor.unwrap_or_else(||{
            if self.enable_tracing{
                ClientInterceptor::with_tracing()
            }else{
                ClientInterceptor::new()
            }
        });
        // 构建客户端
        let inner = UserServiceGrpcClient::with_interceptor(channel, interceptor);

        Ok(UserClient {
            inner,
            max_concurrency,
            semaphore,
            max_retry,
        })
    }
}

#[cfg(test)]
mod test {
    use crate::client::{UserClient, UserClientBuilder};
    use crate::interceptor::client_interceptor::ClientInterceptor;
    use proto::user::GetUserRequest;
    use proto::user::user_service_grpc_client::UserServiceGrpcClient;
    use std::time::Duration;
    use tracing::info;

    #[tokio::test]
    async fn test_use_user_client() {
        let mut client = UserClient::connect("http://127.0.0.1:50051")
            .await
            .expect("get user-client failed");
        let user = client
            .get_user("1".to_string())
            .await
            .expect("get_user response failed");

        info!(
            "user name = {}",
            user.user.ok_or("user not found").unwrap().username
        );
    }

    #[tokio::test]
    async fn test_use_grpc_client() {
        let mut client = UserServiceGrpcClient::connect("http://127.0.0.1:50051")
            .await
            .expect("user grpc service not found");

        let request = tonic::Request::new(GetUserRequest {
            id: "1".to_string(),
        });
        let response = client.get_user(request).await.expect("User not found!");

        info!("RESPONSE={:?}", response);
    }

    #[tokio::test]
    async fn test_use_grpc_client_builder() {
        let interceptor = ClientInterceptor::with_token("my-token");
        let mut client = UserClientBuilder::new()
            .endpoint("http://127.0.0.1:50051")
            .timeout(Duration::from_secs(5))
            .connect_timeout(Duration::from_secs(3))
            .interceptor(interceptor)
            .tcp_keepalive(Duration::from_secs(3))
            .enable_tracing(true)
            .build()
            .await
            .expect("user grpc service not found");

        let response = client
            .get_user("1".to_string())
            .await
            .expect("User not found!");

        info!("RESPONSE={:?}", response);
    }
}
