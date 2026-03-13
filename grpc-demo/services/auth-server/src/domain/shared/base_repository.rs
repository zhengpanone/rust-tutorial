// src/repository/base_repository.rs
use sqlx::PgPool;
use std::marker::PhantomData;
use std::sync::Arc;

#[derive(Clone)]
pub struct BaseRepository<T> {
    pool: Arc<PgPool>,
    _marker: PhantomData<T>,
}

impl<T> BaseRepository<T> {
    pub fn new(pool: Arc<PgPool>) -> Self {
        Self {
            pool,
            _marker: PhantomData,
        }
    }

    pub fn pool(&self) -> &PgPool {
        &self.pool
    }
}


