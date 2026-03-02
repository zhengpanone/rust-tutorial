// src/domain/shared/id.rs

use common::error::{AppError, AppResult};
use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};
use std::marker::PhantomData;
use std::str::FromStr;
use uuid::Uuid;

pub trait DomainId: Sized {
    fn from_string(value: String) -> Self;

    fn as_str(&self) -> &str;

    fn parse(id: &str) -> AppResult<Self> {
        Uuid::parse_str(id)
            .map_err(|_| AppError::InvalidId(id.to_string()))
            .map(|_| Self::from_string(id.to_string()))
    }

    fn new() -> Self {
        Self::from_string(Uuid::new_v4().to_string())
    }
}

/// 泛型领域 ID
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Id<T> {
    value: String,

    #[serde(skip)]
    _marker: PhantomData<T>,
}

impl<T> Id<T> {
    /// 生成新 UUID
    pub fn new() -> Self {
        Self {
            value: Uuid::new_v4().to_string(),
            _marker: PhantomData,
        }
    }

    /// 解析 UUID 字符串
    pub fn parse(id: &str) -> AppResult<Self> {
        Uuid::parse_str(id).map_err(|_| AppError::InvalidId(id.to_string()))?;

        Ok(Self {
            value: id.to_string(),
            _marker: PhantomData,
        })
    }

    /// 获取字符串
    pub fn as_str(&self) -> &str {
        &self.value
    }

    /// 转换为 Uuid
    pub fn to_uuid(&self) -> AppResult<Uuid> {
        Uuid::parse_str(&self.value).map_err(|_| AppError::InvalidId(self.value.clone()))
    }
}

impl<T> Display for Id<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}

impl<T> FromStr for Id<T> {
    type Err = AppError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::parse(s)
    }
}

impl<T> From<Uuid> for Id<T> {
    fn from(value: Uuid) -> Self {
        Self {
            value: value.to_string(),
            _marker: PhantomData,
        }
    }
}

