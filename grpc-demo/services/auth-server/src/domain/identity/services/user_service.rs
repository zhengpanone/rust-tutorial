use crate::domain::identity::models::user::User;
use crate::domain::identity::repository::UserRepository;
use crate::domain::identity::value_objects::email::Email;
use chrono::Utc;
use common::error::AppResult;
use proto::user::UserRole;
use crate::domain::identity::Username;

/// 领域服务 - 封装核心业务逻辑
/// 职责：处理跨聚合的业务规则、复杂的领域逻辑
#[derive(Clone)]
pub struct DomainUserService<R: UserRepository> {
    user_repository: R,
}

impl<R: UserRepository> DomainUserService<R> {
    pub fn new(user_repository: R) -> Self {
        Self { user_repository }
    }

    /// 检查邮箱是否已被注册
    /// 这是一个典型的领域服务方法，因为：
    /// 1. 需要访问仓储
    /// 2. 封装了"邮箱唯一性"这个业务规则
    pub async fn is_email_registered(&self, email: &Email) -> AppResult<bool> {
        let user = self.user_repository.find_by_email(email).await?;
        Ok(user.is_some())
    }

    /// 检查用户名是否可用
    pub async fn is_username_available(&self, username: &Username) -> AppResult<bool> {
        let user = self.user_repository.find_by_username(username).await?;
        Ok(user.is_none())
    }

    /// 验证用户资料（复杂的业务规则）
    pub async fn validate_user_profile(&self, user: &User) -> AppResult<()> {
        // 1. 年龄验证
        if let Some(dob) = user.date_of_birth() {
            let age = Utc::now().year() - dob.year();
            if age < 13 {
                return Err(IdentityError::UnderageUser.into());
            }
        }

        // 2. 用户名格式验证
        if !self.is_valid_username_format(&user.username) {
            return Err(IdentityError::InvalidUsernameFormat.into());
        }

        // 3. 检查是否与其他用户冲突
        if let Some(existing) = self
            .user_repository
            .find_conflicting_user(user.id(), &user.username, &user.email)
            .await?
        {
            return Err(IdentityError::UserConflict {
                field: if existing.email() == &user.email {
                    "email"
                } else {
                    "username"
                }
                .into(),
            }
            .into());
        }

        Ok(())
    }

    /// 计算用户信誉分数（复杂的业务逻辑）
    pub fn calculate_reputation_score(&self, user: &User) -> f64 {
        let mut score = 0.0;

        // 基于多个因素计算
        score += user.contribution_count() as f64 * 0.5;
        score += user.successful_transactions() as f64 * 2.0;
        score -= user.violation_count() as f64 * 5.0;

        // 注册时间加分
        let registration_days = (Utc::now() - user.created_at()).num_days();
        score += (registration_days as f64 / 365.0) * 10.0;

        // 活动频率加分
        if user.is_active() {
            score += 20.0;
        }

        score.max(0.0).min(100.0)
    }

    /// 验证用户名格式
    fn is_valid_username_format(&self, username: &str) -> bool {
        // 长度检查
        if username.len() < 3 || username.len() > 20 {
            return false;
        }

        // 字符检查：只允许字母、数字、下划线
        if !username.chars().all(|c| c.is_alphanumeric() || c == '_') {
            return false;
        }

        // 不能以数字开头
        if username.chars().next().unwrap().is_numeric() {
            return false;
        }

        true
    }

    /// 判断用户是否有权限执行操作
    pub fn can_perform_action(&self, user: &User, action: &str, resource: &str) -> bool {
        match (user.role(), action, resource) {
            // 管理员可以做任何事
            (UserRole::Admin, _, _) => true,

            // 普通用户权限
            (UserRole::User, "read", _) => true,
            (UserRole::User, "update", "own_profile") => true,
            (UserRole::User, "delete", "own_account") => true,

            // VIP用户额外权限
            (UserRole::Vip, "create", "premium_content") => true,
            (UserRole::Vip, "invite", "other_users") => true,

            _ => false,
        }
    }

    /// 推荐相似用户（推荐算法）
    pub async fn recommend_similar_users(&self, user: &User, limit: usize) -> AppResult<Vec<User>> {
        // 1. 获取用户的标签/兴趣
        let user_tags = user.tags();

        // 2. 查找有相似标签的用户
        let similar_users = self
            .user_repository
            .find_users_with_similar_tags(&user_tags, user.id(), limit)
            .await?;

        // 3. 过滤掉被屏蔽的用户
        let blocked_users = user.blocked_users();
        let filtered_users: Vec<User> = similar_users
            .into_iter()
            .filter(|u| !blocked_users.contains(&u.id()))
            .collect();

        Ok(filtered_users)
    }
}
