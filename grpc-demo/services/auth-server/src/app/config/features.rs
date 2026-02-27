// src/app/config/features.rs
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use uuid::Uuid;
use validator::Validate;

/// 功能开关配置
#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct FeaturesConfig {
    /// 启用用户注册
    #[serde(default = "default_true")]
    pub enable_user_registration: bool,

    /// 启用邮箱验证
    #[serde(default = "default_true")]
    pub enable_email_verification: bool,

    /// 启用手机验证
    pub enable_phone_verification: bool,

    /// 启用多因素认证
    pub enable_multi_factor_auth: bool,

    /// 启用社交登录
    pub enable_social_login: bool,

    /// 启用API速率限制
    #[serde(default = "default_true")]
    pub enable_api_rate_limiting: bool,

    /// 启用审计日志
    #[serde(default = "default_true")]
    pub enable_audit_logging: bool,

    /// 启用实时通知
    pub enable_realtime_notifications: bool,

    /// 启用暗黑模式
    #[serde(default = "default_true")]
    pub enable_dark_mode: bool,

    /// 启用实验性功能
    pub enable_experimental_features: bool,

    /// 启用API文档
    #[serde(default = "default_true")]
    pub enable_api_documentation: bool,

    /// 启用指标收集
    #[serde(default = "default_true")]
    pub enable_metrics_collection: bool,

    /// 启用分布式追踪
    pub enable_distributed_tracing: bool,

    /// 启用缓存
    #[serde(default = "default_true")]
    pub enable_caching: bool,

    /// 启用消息队列
    pub enable_message_queue: bool,

    /// 启用任务队列
    pub enable_task_queue: bool,

    /// 启用文件上传
    #[serde(default = "default_true")]
    pub enable_file_upload: bool,

    /// 启用邮件服务
    pub enable_email_service: bool,

    /// 启用短信服务
    pub enable_sms_service: bool,

    /// 启用WebSocket
    pub enable_websocket: bool,

    /// 启用gRPC
    pub enable_grpc: bool,

    /// 功能开关映射表
    #[serde(default)]
    pub feature_flags: HashMap<String, FeatureFlag>,
}

impl Default for FeaturesConfig {
    fn default() -> Self {
        Self {
            enable_user_registration: true,
            enable_email_verification: true,
            enable_phone_verification: false,
            enable_multi_factor_auth: false,
            enable_social_login: false,
            enable_api_rate_limiting: true,
            enable_audit_logging: true,
            enable_realtime_notifications: false,
            enable_dark_mode: true,
            enable_experimental_features: false,
            enable_api_documentation: true,
            enable_metrics_collection: true,
            enable_distributed_tracing: false,
            enable_caching: true,
            enable_message_queue: false,
            enable_task_queue: false,
            enable_file_upload: true,
            enable_email_service: false,
            enable_sms_service: false,
            enable_websocket: false,
            enable_grpc: false,
            feature_flags: HashMap::new(),
        }
    }
}

/// 功能开关
#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct FeatureFlag {
    /// 功能名称
    pub name: String,

    /// 功能描述
    pub description: Option<String>,

    /// 是否启用
    pub enabled: bool,

    /// 功能类型
    pub feature_type: FeatureType,

    /// 目标受众
    pub target: FeatureTarget,

    /// 发布策略
    pub rollout_strategy: RolloutStrategy,

    /// 规则
    #[serde(default)]
    pub rules: Vec<FeatureRule>,

    /// 元数据
    #[serde(default)]
    pub metadata: HashMap<String, serde_json::Value>,

    /// 创建时间
    pub created_at: DateTime<Utc>,

    /// 更新时间
    pub updated_at: DateTime<Utc>,

    /// 有效时间
    pub valid_from: Option<DateTime<Utc>>,
    pub valid_until: Option<DateTime<Utc>>,
}

/// 功能类型
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum FeatureType {
    /// 功能开关
    #[serde(rename = "toggle")]
    Toggle,

    /// 发布开关
    #[serde(rename = "release")]
    Release,

    /// 实验功能
    #[serde(rename = "experiment")]
    Experiment,

    /// 权限控制
    #[serde(rename = "permission")]
    Permission,

    /// 运维开关
    #[serde(rename = "ops")]
    Ops,
}

/// 功能目标
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FeatureTarget {
    /// 目标环境
    pub environments: HashSet<String>,

    /// 目标用户组
    pub user_groups: HashSet<String>,

    /// 目标用户ID
    pub user_ids: HashSet<Uuid>,

    /// 目标IP地址
    pub ip_addresses: HashSet<String>,

    /// 目标地区
    pub regions: HashSet<String>,

    /// 目标客户端
    pub clients: HashSet<String>,

    /// 百分比发布 (0-100)
    pub percentage: u8,
}

impl Default for FeatureTarget {
    fn default() -> Self {
        Self {
            environments: HashSet::from(["all".to_string()]),
            user_groups: HashSet::new(),
            user_ids: HashSet::new(),
            ip_addresses: HashSet::new(),
            regions: HashSet::new(),
            clients: HashSet::new(),
            percentage: 100,
        }
    }
}

/// 发布策略
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RolloutStrategy {
    /// 策略类型
    pub strategy_type: RolloutType,

    /// 渐进式发布阶段
    pub stages: Vec<RolloutStage>,

    /// 当前阶段索引
    pub current_stage: usize,

    /// 是否自动推进
    pub auto_advance: bool,

    /// 自动推进间隔（小时）
    pub advance_interval_hours: u32,
}

/// 发布类型
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum RolloutType {
    /// 立即发布
    #[serde(rename = "instant")]
    Instant,

    /// 渐进式发布
    #[serde(rename = "gradual")]
    Gradual,

    /// 金丝雀发布
    #[serde(rename = "canary")]
    Canary,

    /// A/B测试
    #[serde(rename = "ab_test")]
    AbTest,
}

/// 发布阶段
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RolloutStage {
    /// 阶段名称
    pub name: String,

    /// 目标百分比
    pub percentage: u8,

    /// 开始时间
    pub start_time: Option<DateTime<Utc>>,

    /// 持续时间（小时）
    pub duration_hours: u32,

    /// 成功条件
    pub success_conditions: Vec<SuccessCondition>,
}

/// 成功条件
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SuccessCondition {
    /// 指标名称
    pub metric: String,

    /// 比较操作符
    pub operator: ConditionOperator,

    /// 目标值
    pub target_value: f64,

    /// 检查窗口（小时）
    pub window_hours: u32,
}

/// 条件操作符
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ConditionOperator {
    #[serde(rename = "gt")]
    GreaterThan,
    #[serde(rename = "gte")]
    GreaterThanOrEqual,
    #[serde(rename = "lt")]
    LessThan,
    #[serde(rename = "lte")]
    LessThanOrEqual,
    #[serde(rename = "eq")]
    Equal,
    #[serde(rename = "neq")]
    NotEqual,
}

/// 功能规则
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FeatureRule {
    /// 规则名称
    pub name: String,

    /// 规则条件
    pub condition: RuleCondition,

    /// 规则动作
    pub action: RuleAction,

    /// 规则优先级
    pub priority: u32,

    /// 是否启用
    pub enabled: bool,
}

/// 规则条件
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RuleCondition {
    /// 总是匹配
    Always,

    /// 匹配用户ID
    UserId(Uuid),

    /// 匹配用户组
    UserGroup(String),

    /// 匹配IP地址
    IpAddress(String),

    /// 匹配地区
    Region(String),

    /// 匹配客户端
    Client(String),

    /// 百分比匹配
    Percentage(u8),

    /// 时间范围
    TimeRange {
        start: DateTime<Utc>,
        end: DateTime<Utc>,
    },

    /// 表达式
    Expression(String),

    /// 组合条件
    And(Vec<RuleCondition>),
    Or(Vec<RuleCondition>),
    Not(Box<RuleCondition>),
}

/// 规则动作
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RuleAction {
    /// 启用功能
    Enable,

    /// 禁用功能
    Disable,

    /// 返回自定义值
    ReturnValue(serde_json::Value),

    /// 重定向
    Redirect(String),

    /// 抛出异常
    ThrowError(String),
}

fn default_true() -> bool {
    true
}
