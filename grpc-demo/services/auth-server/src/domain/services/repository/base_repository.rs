// src/repository/base_repository.rs
use sqlx::{PgPool, postgres::PgQueryResult};
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

pub struct PaginationParams {
    pub page: i64,
    pub page_size: i64,
    pub offset: i64,
}

impl PaginationParams {
    pub fn new(page: Option<i64>, page_size: Option<i64>) -> Self {
        let page = page.unwrap_or(1).max(1);
        let page_size = page_size.unwrap_or(10).max(1).min(100);
        let offset = (page - 1) * page_size;

        Self {
            page,
            page_size,
            offset,
        }
    }
}
