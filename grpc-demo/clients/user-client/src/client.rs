use crate::error::UserClientError;
use crate::interceptor::ClientInterceptor;
use proto::user::user_service_grpc_client::UserServiceGrpcClient;
use proto::user::{GetUserRequest, UserResponse};
use std::time::Duration;
use tonic::codegen::InterceptedService;
use tonic::transport::{Channel, Endpoint};
use tracing::debug;
use tracing_attributes::instrument;

#[derive(Clone)]
pub struct UserClient {
    inner: UserServiceGrpcClient<InterceptedService<Channel, ClientInterceptor>>,
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
        Self {
            inner: UserServiceGrpcClient::with_interceptor(channel, interceptor),
        }
    }

    /// 获取用户（示例方法）
    ///  #[instrument(skip(self))] 是 tracing crate 提供的一个 宏属性
    /// tracing：核心库，提供 info!, debug!, span! 等宏
    /// tracing-attributes：提供宏属性 #[instrument]，可以直接加在函数上
    /// skip(self) 如果函数参数里有不想打印或不可打印的类型（比如 &self 或 &mut self），可以用 skip(...)
    #[instrument(skip(self))]
    pub async fn get_user(&mut self, id: String) -> Result<UserResponse, UserClientError> {
        let request = GetUserRequest { id };
        debug!("get user request {:?}", request);
        let response = self.inner.get_user(request).await?;

        Ok(response.into_inner())
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
        // 构建 Channel
        let channel = ep.connect().await?;
        // 拦截器
        let interceptor = self.interceptor.unwrap_or_else(ClientInterceptor::new);
        // 构建客户端
        let inner = UserServiceGrpcClient::with_interceptor(channel, interceptor);

        Ok(UserClient { inner })
    }
}

#[cfg(test)]
mod test {
    use crate::client::{UserClient, UserClientBuilder};
    use crate::interceptor::ClientInterceptor;
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
