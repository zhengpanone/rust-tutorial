// src/app/config/message_queue.rs
use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use validator::Validate;

/// 消息队列配置
#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct MessageQueueConfig {
    /// 是否启用消息队列
    pub enabled: bool,

    /// 消息队列提供者
    pub provider: MessageQueueProvider,

    /// 连接配置
    pub connection: ConnectionConfig,

    /// 生产者配置
    pub producer: ProducerConfig,

    /// 消费者配置
    pub consumer: ConsumerConfig,

    /// 交换机配置
    pub exchanges: Vec<ExchangeConfig>,

    /// 队列配置
    pub queues: Vec<QueueConfig>,

    /// 绑定配置
    pub bindings: Vec<BindingConfig>,

    /// 重试策略
    pub retry_policy: RetryPolicy,

    /// 监控配置
    pub monitoring: MonitoringConfig,

    /// 死信队列配置
    pub dead_letter_queue: Option<DeadLetterQueueConfig>,

    /// 消息序列化配置
    pub serialization: SerializationConfig,

    // 安全配置
    // pub security: SecurityConfig,
}

/// 消息队列提供者
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum MessageQueueProvider {
    #[serde(rename = "rabbitmq")]
    RabbitMQ,

    #[serde(rename = "kafka")]
    Kafka,

    #[serde(rename = "redis")]
    Redis,

    #[serde(rename = "nats")]
    NATS,

    #[serde(rename = "sqs")]
    SQS,

    #[serde(rename = "pulsar")]
    Pulsar,

    #[serde(rename = "memory")]
    Memory,
}

impl MessageQueueProvider {
    pub fn to_string(&self) -> String {
        match self {
            MessageQueueProvider::RabbitMQ => "rabbitmq".to_string(),
            MessageQueueProvider::Kafka => "kafka".to_string(),
            MessageQueueProvider::Redis => "redis".to_string(),
            MessageQueueProvider::NATS => "nats".to_string(),
            MessageQueueProvider::SQS => "sqs".to_string(),
            MessageQueueProvider::Pulsar => "pulsar".to_string(),
            MessageQueueProvider::Memory => "memory".to_string(),
        }
    }

    pub fn from_str(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "rabbitmq" => Some(MessageQueueProvider::RabbitMQ),
            "kafka" => Some(MessageQueueProvider::Kafka),
            "redis" => Some(MessageQueueProvider::Redis),
            "nats" => Some(MessageQueueProvider::NATS),
            "sqs" => Some(MessageQueueProvider::SQS),
            "pulsar" => Some(MessageQueueProvider::Pulsar),
            "memory" => Some(MessageQueueProvider::Memory),
            _ => None,
        }
    }
}

/// 连接配置
#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct ConnectionConfig {
    /// 连接URL
    #[validate(url)]
    pub url: String,

    /// 连接池大小
    #[validate(range(min = 1, max = 100))]
    pub pool_size: u32,

    /// 连接超时（秒）
    pub connection_timeout_seconds: u64,

    /// 心跳间隔（秒）
    pub heartbeat_interval_seconds: u64,

    /// 最大帧大小（字节）
    pub max_frame_size_bytes: usize,

    /// 是否启用TLS
    pub enable_tls: bool,

    /// TLS配置
    pub tls: Option<TlsConfig>,

    /// 认证配置
    pub authentication: Option<AuthenticationConfig>,

    /// 虚拟主机
    pub virtual_host: String,

    /// 连接重试配置
    pub retry: ConnectionRetryConfig,

    /// 是否启用连接池
    pub enable_connection_pool: bool,

    /// 连接池最大空闲时间（秒）
    pub connection_pool_max_idle_seconds: u64,
}

/// 生产者配置
#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct ProducerConfig {
    /// 生产者ID
    pub producer_id: String,

    /// 确认模式
    pub confirm_mode: ConfirmMode,

    /// 批处理大小
    pub batch_size: u32,

    /// 批处理超时（毫秒）
    pub batch_timeout_ms: u64,

    /// 最大重试次数
    pub max_retries: u32,

    /// 压缩配置
    pub compression: CompressionConfig,

    /// 分区策略
    pub partitioning: PartitioningStrategy,

    /// 消息超时（秒）
    pub message_timeout_seconds: u64,

    /// 是否启用幂等生产者
    pub enable_idempotent_producer: bool,

    /// 事务配置
    pub transaction: TransactionConfig,
}

/// 消费者配置
#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct ConsumerConfig {
    /// 消费者组
    pub consumer_group: String,

    /// 预取数量
    #[validate(range(min = 1, max = 1000))]
    pub prefetch_count: u32,

    /// 确认模式
    pub ack_mode: AckMode,

    /// 自动确认
    pub auto_ack: bool,

    /// 最大重试次数
    pub max_retries: u32,

    /// 重试延迟策略
    pub retry_delay_strategy: RetryDelayStrategy,

    /// 并发消费者数量
    pub concurrent_consumers: u32,

    /// 批处理大小
    pub batch_size: u32,

    /// 批处理超时（毫秒）
    pub batch_timeout_ms: u64,

    /// 是否启用死信队列
    pub enable_dead_letter_queue: bool,

    /// 是否启用消息追踪
    pub enable_message_tracing: bool,

    /// 偏移量提交间隔（毫秒）
    pub offset_commit_interval_ms: u64,

    /// 偏移量提交策略
    pub offset_commit_strategy: OffsetCommitStrategy,
}

/// 交换机配置
#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct ExchangeConfig {
    /// 交换机名称
    pub name: String,

    /// 交换机类型
    pub exchange_type: ExchangeType,

    /// 是否持久化
    pub durable: bool,

    /// 是否自动删除
    pub auto_delete: bool,

    /// 是否延迟
    pub delayed: bool,

    /// 是否内部交换机
    pub internal: bool,

    /// 参数
    pub arguments: HashMap<String, serde_json::Value>,
}

/// 队列配置
#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct QueueConfig {
    /// 队列名称
    pub name: String,

    /// 是否持久化
    pub durable: bool,

    /// 是否排他
    pub exclusive: bool,

    /// 是否自动删除
    pub auto_delete: bool,

    /// 最大长度
    pub max_length: Option<u32>,

    /// 消息TTL（秒）
    pub message_ttl_seconds: Option<u64>,

    /// 最大优先级
    pub max_priority: Option<u8>,

    /// 死信交换机
    pub dead_letter_exchange: Option<String>,

    /// 死信路由键
    pub dead_letter_routing_key: Option<String>,

    /// 参数
    pub arguments: HashMap<String, serde_json::Value>,
}

/// 绑定配置
#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct BindingConfig {
    /// 交换机名称
    pub exchange: String,

    /// 队列名称
    pub queue: String,

    /// 路由键
    pub routing_key: String,

    /// 参数
    pub arguments: HashMap<String, serde_json::Value>,
}

/// TLS配置
#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct TlsConfig {
    /// CA证书路径
    pub ca_cert_path: String,

    /// 客户端证书路径
    pub client_cert_path: String,

    /// 客户端密钥路径
    pub client_key_path: String,

    /// 是否验证服务器证书
    pub verify_server_cert: bool,

    /// 是否验证主机名
    pub verify_hostname: bool,

    /// TLS版本
    pub tls_version: String,

    /// 密码套件
    pub cipher_suites: Vec<String>,
}

/// 认证配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthenticationConfig {
    /// 认证类型
    pub auth_type: AuthType,

    /// 用户名
    pub username: String,

    /// 密码
    pub password: String,

    /// 机制
    pub mechanism: String,

    /// 令牌
    pub token: Option<String>,

    /// 访问密钥
    pub access_key: Option<String>,

    /// 秘密密钥
    pub secret_key: Option<String>,
}

/// 认证类型
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AuthType {
    #[serde(rename = "plain")]
    Plain,

    #[serde(rename = "oauth2")]
    OAuth2,

    #[serde(rename = "aws")]
    Aws,

    #[serde(rename = "token")]
    Token,
}

/// 确认模式
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ConfirmMode {
    #[serde(rename = "none")]
    None,

    #[serde(rename = "simple")]
    Simple,

    #[serde(rename = "batch")]
    Batch,

    #[serde(rename = "tx")]
    Transactional,
}

/// 压缩配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompressionConfig {
    /// 压缩算法
    pub algorithm: CompressionAlgorithm,

    /// 压缩级别
    pub level: u32,

    /// 最小压缩大小（字节）
    pub min_size_bytes: usize,
}

/// 压缩算法
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum CompressionAlgorithm {
    #[serde(rename = "none")]
    None,

    #[serde(rename = "gzip")]
    Gzip,

    #[serde(rename = "snappy")]
    Snappy,

    #[serde(rename = "lz4")]
    Lz4,

    #[serde(rename = "zstd")]
    Zstd,
}

/// 分区策略
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PartitioningStrategy {
    #[serde(rename = "round_robin")]
    RoundRobin,

    #[serde(rename = "hash")]
    Hash(String),

    #[serde(rename = "consistent_hash")]
    ConsistentHash(u32),

    #[serde(rename = "key")]
    Key(String),

    #[serde(rename = "custom")]
    Custom(String),
}

/// 事务配置
#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct TransactionConfig {
    /// 是否启用事务
    pub enabled: bool,

    /// 事务超时（秒）
    pub timeout_seconds: u64,

    /// 最大重试次数
    pub max_retries: u32,

    /// 回滚策略
    pub rollback_strategy: RollbackStrategy,
}

/// 回滚策略
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum RollbackStrategy {
    #[serde(rename = "always")]
    Always,

    #[serde(rename = "on_error")]
    OnError,

    #[serde(rename = "never")]
    Never,
}

/// 确认模式
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AckMode {
    #[serde(rename = "auto")]
    Auto,

    #[serde(rename = "manual")]
    Manual,

    #[serde(rename = "none")]
    None,
}

/// 重试延迟策略
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RetryDelayStrategy {
    #[serde(rename = "fixed")]
    Fixed(u64), // 固定延迟（毫秒）

    #[serde(rename = "exponential")]
    Exponential {
        initial_delay_ms: u64,
        multiplier: f64,
        max_delay_ms: u64,
    },

    #[serde(rename = "random")]
    Random {
        min_delay_ms: u64,
        max_delay_ms: u64,
    },
}

/// 偏移量提交策略
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum OffsetCommitStrategy {
    #[serde(rename = "sync")]
    Synchronous,

    #[serde(rename = "async")]
    Asynchronous,

    #[serde(rename = "periodic")]
    Periodic,
}

/// 重试策略
#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct RetryPolicy {
    /// 最大重试次数
    pub max_retries: u32,

    /// 初始重试延迟（秒）
    pub initial_retry_delay_seconds: u64,

    /// 最大重试延迟（秒）
    pub max_retry_delay_seconds: u64,

    /// 退避乘数
    pub backoff_multiplier: f64,

    /// 是否启用抖动
    pub enable_jitter: bool,

    /// 重试条件
    pub retry_conditions: Vec<RetryCondition>,

    /// 永不重试的错误
    pub non_retryable_errors: Vec<String>,
}

/// 重试条件
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RetryCondition {
    #[serde(rename = "timeout")]
    Timeout,

    #[serde(rename = "network_error")]
    NetworkError,

    #[serde(rename = "server_error")]
    ServerError,

    #[serde(rename = "transient_error")]
    TransientError,

    #[serde(rename = "custom")]
    Custom(String),
}

/// 监控配置
#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct MonitoringConfig {
    /// 是否启用监控
    pub enabled: bool,

    /// 指标收集间隔（秒）
    pub metrics_collection_interval_seconds: u64,

    /// 监控指标
    pub metrics: Vec<MetricConfig>,

    /// 告警配置
    pub alerts: Vec<AlertConfig>,

    /// 追踪配置
    pub tracing: TracingConfig,
}

/// 指标配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MetricConfig {
    /// 指标名称
    pub name: String,

    /// 指标类型
    pub metric_type: MetricType,

    /// 指标标签
    pub labels: HashMap<String, String>,

    /// 收集频率（秒）
    pub collection_frequency_seconds: u64,

    /// 聚合窗口（秒）
    pub aggregation_window_seconds: u64,
}

/// 指标类型
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum MetricType {
    #[serde(rename = "counter")]
    Counter,

    #[serde(rename = "gauge")]
    Gauge,

    #[serde(rename = "histogram")]
    Histogram,

    #[serde(rename = "summary")]
    Summary,
}

/// 告警配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AlertConfig {
    /// 告警名称
    pub name: String,

    /// 告警条件
    pub condition: String,

    /// 严重程度
    pub severity: AlertSeverity,

    /// 告警接收者
    pub receivers: Vec<String>,

    /// 冷却时间（秒）
    pub cooldown_seconds: u64,

    /// 是否启用
    pub enabled: bool,
}

/// 告警严重程度
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AlertSeverity {
    #[serde(rename = "critical")]
    Critical,

    #[serde(rename = "error")]
    Error,

    #[serde(rename = "warning")]
    Warning,

    #[serde(rename = "info")]
    Info,
}

/// 追踪配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TracingConfig {
    /// 是否启用追踪
    pub enabled: bool,

    /// 采样率 (0.0-1.0)
    pub sampling_rate: f64,

    /// 是否记录消息体
    pub log_message_body: bool,

    /// 追踪头
    pub trace_headers: Vec<String>,
}

/// 死信队列配置
#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct DeadLetterQueueConfig {
    /// 是否启用死信队列
    pub enabled: bool,

    /// 死信交换机
    pub dead_letter_exchange: String,

    /// 死信队列
    pub dead_letter_queue: String,

    /// 消息TTL（秒）
    pub message_ttl_seconds: u64,

    /// 最大重试次数
    pub max_redelivery_count: u32,

    /// 重试延迟策略
    pub retry_delay_strategy: RetryDelayStrategy,

    /// 告警配置
    pub alerts: Vec<AlertConfig>,
}

/// 消息序列化配置
#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct SerializationConfig {
    /// 序列化格式
    pub format: SerializationFormat,

    /// 是否压缩
    pub compress: bool,

    /// 压缩算法
    pub compression_algorithm: CompressionAlgorithm,

    /// 压缩级别
    pub compression_level: u32,

    /// 是否加密
    pub encrypt: bool,

    /// 加密算法
    pub encryption_algorithm: EncryptionAlgorithm,

    /// 是否签名
    pub sign: bool,

    /// 签名算法
    pub signature_algorithm: SignatureAlgorithm,

    /// 消息版本
    pub message_version: String,

    /// 兼容性配置
    pub compatibility: CompatibilityConfig,
}

/// 序列化格式
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SerializationFormat {
    #[serde(rename = "json")]
    Json,

    #[serde(rename = "protobuf")]
    Protobuf,

    #[serde(rename = "avro")]
    Avro,

    #[serde(rename = "messagepack")]
    MessagePack,

    #[serde(rename = "bincode")]
    Bincode,

    #[serde(rename = "yaml")]
    Yaml,
}

/// 加密算法
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum EncryptionAlgorithm {
    #[serde(rename = "aes-256-gcm")]
    Aes256Gcm,

    #[serde(rename = "chacha20-poly1305")]
    Chacha20Poly1305,

    #[serde(rename = "none")]
    None,
}

/// 签名算法
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SignatureAlgorithm {
    #[serde(rename = "ed25519")]
    Ed25519,

    #[serde(rename = "rsa-pss")]
    RsaPss,

    #[serde(rename = "ecdsa")]
    Ecdsa,

    #[serde(rename = "hmac")]
    Hmac,

    #[serde(rename = "none")]
    None,
}

/// 兼容性配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompatibilityConfig {
    /// 兼容性模式
    pub compatibility_mode: CompatibilityMode,

    /// 向后兼容版本
    pub backward_compatible_versions: Vec<String>,

    /// 向前兼容版本
    pub forward_compatible_versions: Vec<String>,

    /// 严格模式
    pub strict_mode: bool,
}

/// 兼容性模式
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum CompatibilityMode {
    #[serde(rename = "none")]
    None,

    #[serde(rename = "backward")]
    Backward,

    #[serde(rename = "forward")]
    Forward,

    #[serde(rename = "full")]
    Full,
}

/// 安全配置
#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct SecurityConfig {
    /// 是否启用SSL/TLS
    pub enable_ssl: bool,

    /// 是否启用认证
    pub enable_authentication: bool,

    /// 是否启用授权
    pub enable_authorization: bool,

    /// 访问控制列表
    pub acl: Vec<AccessControlRule>,

    /// 审计配置
    pub audit: AuditConfig,

    // /// 加密配置
    // pub encryption: EncryptionConfig,

    /// 签名配置
    pub signature: SignatureConfig,
}

/// 访问控制规则
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccessControlRule {
    /// 主体
    pub principal: String,

    /// 资源
    pub resource: String,

    /// 操作
    pub operation: Operation,

    /// 是否允许
    pub allow: bool,

    /// 条件
    pub condition: Option<String>,
}

/// 操作类型
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Operation {
    #[serde(rename = "read")]
    Read,

    #[serde(rename = "write")]
    Write,

    #[serde(rename = "create")]
    Create,

    #[serde(rename = "delete")]
    Delete,

    #[serde(rename = "configure")]
    Configure,

    #[serde(rename = "all")]
    All,
}

/// 审计配置
#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct AuditConfig {
    /// 是否启用审计
    pub enabled: bool,

    /// 审计级别
    pub audit_level: AuditLevel,

    /// 审计事件
    pub events: Vec<AuditEvent>,

    /// 审计存储
    pub storage: AuditStorage,
}

/// 审计级别
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AuditLevel {
    #[serde(rename = "none")]
    None,

    #[serde(rename = "basic")]
    Basic,

    #[serde(rename = "detailed")]
    Detailed,

    #[serde(rename = "full")]
    Full,
}

/// 审计事件
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AuditEvent {
    #[serde(rename = "connection")]
    Connection,

    #[serde(rename = "authentication")]
    Authentication,

    #[serde(rename = "authorization")]
    Authorization,

    #[serde(rename = "message_publish")]
    MessagePublish,

    #[serde(rename = "message_consume")]
    MessageConsume,

    #[serde(rename = "queue_operation")]
    QueueOperation,

    #[serde(rename = "exchange_operation")]
    ExchangeOperation,

    #[serde(rename = "admin_operation")]
    AdminOperation,
}

/// 审计存储
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuditStorage {
    /// 存储类型
    pub storage_type: StorageType,

    /// 存储路径/URL
    pub storage_path: String,

    /// 保留天数
    pub retention_days: u32,

    /// 最大存储大小
    pub max_storage_size_mb: u64,

    /// 压缩配置
    pub compression: CompressionConfig,
}

/// 存储类型
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum StorageType {
    #[serde(rename = "file")]
    File,

    #[serde(rename = "database")]
    Database,

    #[serde(rename = "elasticsearch")]
    Elasticsearch,

    #[serde(rename = "s3")]
    S3,
}

// /// 加密配置
// #[derive(Debug, Clone, Serialize, Deserialize)]
// pub struct EncryptionConfig {
//     /// 加密算法
//     pub algorithm: EncryptionAlgorithm,
//
//     /// 密钥管理
//     pub key_management: KeyManagement,
//
//     /// 密钥轮换策略
//     pub key_rotation: KeyRotationPolicy,
// }

/// 密钥管理
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KeyManagement {
    /// 密钥管理类型
    pub key_management_type: KeyManagementType,

    /// 密钥存储
    pub key_store: KeyStore,

    /// 密钥标识
    pub key_id: String,

    /// 密钥版本
    pub key_version: String,
}

/// 密钥管理类型
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum KeyManagementType {
    #[serde(rename = "aws_kms")]
    AwsKms,

    #[serde(rename = "azure_key_vault")]
    AzureKeyVault,

    #[serde(rename = "gcp_kms")]
    GcpKms,

    #[serde(rename = "hashicorp_vault")]
    HashicorpVault,

    #[serde(rename = "local")]
    Local,
}

/// 密钥存储
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KeyStore {
    /// 存储类型
    pub storage_type: StorageType,

    /// 存储路径
    pub storage_path: String,

    // /// 加密配置
    // pub encryption: EncryptionConfig,
}

/// 密钥轮换策略
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KeyRotationPolicy {
    /// 轮换间隔（天）
    pub rotation_interval_days: u32,

    /// 自动轮换
    pub auto_rotate: bool,

    /// 保留旧密钥天数
    pub retain_old_keys_days: u32,
}

/// 签名配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SignatureConfig {
    /// 签名算法
    pub algorithm: SignatureAlgorithm,

    /// 密钥管理
    pub key_management: KeyManagement,

    /// 验证配置
    pub verification: VerificationConfig,
}

/// 验证配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VerificationConfig {
    /// 是否验证签名
    pub verify_signature: bool,

    /// 是否验证时间戳
    pub verify_timestamp: bool,

    /// 时间容差（秒）
    pub time_tolerance_seconds: u64,

    /// 是否验证颁发者
    pub verify_issuer: bool,

    /// 颁发者
    pub issuer: Option<String>,
}

/// 连接重试配置
#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct ConnectionRetryConfig {
    /// 最大重试次数
    pub max_retry_attempts: u32,

    /// 重试间隔（秒）
    pub retry_interval_seconds: u64,

    /// 退避乘数
    pub backoff_multiplier: f64,

    /// 最大重试间隔（秒）
    pub max_retry_interval_seconds: u64,

    /// 是否启用抖动
    pub enable_jitter: bool,

    /// 重试超时（秒）
    pub retry_timeout_seconds: u64,
}

/// 交换机类型
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ExchangeType {
    #[serde(rename = "direct")]
    Direct,

    #[serde(rename = "fanout")]
    Fanout,

    #[serde(rename = "topic")]
    Topic,

    #[serde(rename = "headers")]
    Headers,

    #[serde(rename = "system")]
    System,
}

impl Default for MessageQueueConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            provider: MessageQueueProvider::RabbitMQ,
            connection: ConnectionConfig::default(),
            producer: ProducerConfig::default(),
            consumer: ConsumerConfig::default(),
            exchanges: Vec::new(),
            queues: Vec::new(),
            bindings: Vec::new(),
            retry_policy: RetryPolicy::default(),
            monitoring: MonitoringConfig::default(),
            dead_letter_queue: None,
            serialization: SerializationConfig::default(),
            // security: SecurityConfig::default(),
        }
    }
}

impl Default for ConnectionConfig {
    fn default() -> Self {
        Self {
            url: "amqp://guest:guest@localhost:5672/".to_string(),
            pool_size: 10,
            connection_timeout_seconds: 5,
            heartbeat_interval_seconds: 30,
            max_frame_size_bytes: 131072, // 128KB
            enable_tls: false,
            tls: None,
            authentication: None,
            virtual_host: "/".to_string(),
            retry: ConnectionRetryConfig::default(),
            enable_connection_pool: true,
            connection_pool_max_idle_seconds: 300,
        }
    }
}

impl Default for ProducerConfig {
    fn default() -> Self {
        Self {
            producer_id: "default-producer".to_string(),
            confirm_mode: ConfirmMode::Simple,
            batch_size: 100,
            batch_timeout_ms: 1000,
            max_retries: 3,
            compression: CompressionConfig::default(),
            partitioning: PartitioningStrategy::RoundRobin,
            message_timeout_seconds: 3600,
            enable_idempotent_producer: false,
            transaction: TransactionConfig::default(),
        }
    }
}

impl Default for ConsumerConfig {
    fn default() -> Self {
        Self {
            consumer_group: "default-consumer-group".to_string(),
            prefetch_count: 10,
            ack_mode: AckMode::Manual,
            auto_ack: false,
            max_retries: 3,
            retry_delay_strategy: RetryDelayStrategy::Fixed(1000),
            concurrent_consumers: 1,
            batch_size: 1,
            batch_timeout_ms: 1000,
            enable_dead_letter_queue: true,
            enable_message_tracing: false,
            offset_commit_interval_ms: 5000,
            offset_commit_strategy: OffsetCommitStrategy::Periodic,
        }
    }
}

impl Default for RetryPolicy {
    fn default() -> Self {
        Self {
            max_retries: 3,
            initial_retry_delay_seconds: 1,
            max_retry_delay_seconds: 60,
            backoff_multiplier: 2.0,
            enable_jitter: true,
            retry_conditions: vec![
                RetryCondition::NetworkError,
                RetryCondition::ServerError,
                RetryCondition::TransientError,
            ],
            non_retryable_errors: vec![
                "validation_error".to_string(),
                "authentication_error".to_string(),
                "authorization_error".to_string(),
            ],
        }
    }
}

impl Default for ConnectionRetryConfig {
    fn default() -> Self {
        Self {
            max_retry_attempts: 3,
            retry_interval_seconds: 5,
            backoff_multiplier: 2.0,
            max_retry_interval_seconds: 60,
            enable_jitter: true,
            retry_timeout_seconds: 300,
        }
    }
}

impl Default for CompressionConfig {
    fn default() -> Self {
        Self {
            algorithm: CompressionAlgorithm::None,
            level: 6,
            min_size_bytes: 1024,
        }
    }
}

impl Default for TransactionConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            timeout_seconds: 30,
            max_retries: 3,
            rollback_strategy: RollbackStrategy::OnError,
        }
    }
}

impl Default for MonitoringConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            metrics_collection_interval_seconds: 60,
            metrics: vec![
                MetricConfig {
                    name: "message_published".to_string(),
                    metric_type: MetricType::Counter,
                    labels: HashMap::new(),
                    collection_frequency_seconds: 10,
                    aggregation_window_seconds: 60,
                },
                MetricConfig {
                    name: "message_consumed".to_string(),
                    metric_type: MetricType::Counter,
                    labels: HashMap::new(),
                    collection_frequency_seconds: 10,
                    aggregation_window_seconds: 60,
                },
            ],
            alerts: vec![AlertConfig {
                name: "high_error_rate".to_string(),
                condition: "error_rate > 0.1".to_string(),
                severity: AlertSeverity::Error,
                receivers: vec!["admin@example.com".to_string()],
                cooldown_seconds: 300,
                enabled: true,
            }],
            tracing: TracingConfig {
                enabled: false,
                sampling_rate: 0.1,
                log_message_body: false,
                trace_headers: vec![
                    "trace_id".to_string(),
                    "span_id".to_string(),
                    "parent_id".to_string(),
                ],
            },
        }
    }
}

impl Default for SerializationConfig {
    fn default() -> Self {
        Self {
            format: SerializationFormat::Json,
            compress: false,
            compression_algorithm: CompressionAlgorithm::None,
            compression_level: 6,
            encrypt: false,
            encryption_algorithm: EncryptionAlgorithm::None,
            sign: false,
            signature_algorithm: SignatureAlgorithm::None,
            message_version: "1.0".to_string(),
            compatibility: CompatibilityConfig {
                compatibility_mode: CompatibilityMode::None,
                backward_compatible_versions: Vec::new(),
                forward_compatible_versions: Vec::new(),
                strict_mode: false,
            },
        }
    }
}
