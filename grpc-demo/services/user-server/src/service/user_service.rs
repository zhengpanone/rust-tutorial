use std::sync::Arc;
use crate::entity::user_entity::UserEntity;
use crate::repository::user_repository::UserRepository;
use crate::state::AppState;

#[derive(Clone)]
pub struct UserService {
    state: Arc<AppState>,
    repo: UserRepository,
}

impl UserService {
    pub fn new(state: Arc<AppState>) -> Self {
        let repository = UserRepository::new(state.db.clone());
        Self {
            state,
            repo: repository,
        }
    }

    pub async fn get_user_by_id(&self, id: &str) -> anyhow::Result<Option<UserEntity>> {
        let user = self.repo.find_by_id(id).await?;
        Ok(user)
    }
}
