// src/domain/services/value_objects/url.rs
use serde::{Deserialize, Serialize};
use thiserror::Error;
use url::Url as ExternalUrl;


#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Url {
    value: String,
}

impl Url {
    pub fn new(value: String) -> Result<Self, UrlError> {
        if value.len() > 200 {
            return Err(UrlError::TooLong(value.len()));
        }

        // 验证URL格式
        if !value.is_empty() {
            ExternalUrl::parse(&value).map_err(|_| UrlError::InvalidFormat(value.clone()))?;
        }

        Ok(Self { value })
    }

    pub fn as_str(&self) -> &str {
        &self.value
    }

    pub fn is_empty(&self) -> bool {
        self.value.is_empty()
    }
}

impl From<Url> for String {
    fn from(url: Url) -> Self {
        url.value
    }
}

impl std::fmt::Display for Url {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}

impl Serialize for Url {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&self.value)
    }
}

impl<'de> Deserialize<'de> for Url {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = String::deserialize(deserializer)?;
        Url::new(value).map_err(serde::de::Error::custom)
    }
}

#[derive(Debug, Error)]
pub enum UrlError {
    #[error("URL长度超过限制: {0}")]
    TooLong(usize),

    #[error("URL格式无效: {0}")]
    InvalidFormat(String),
}
