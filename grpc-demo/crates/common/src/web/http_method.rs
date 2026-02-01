use axum::http::Method;
use core::fmt;
use serde::{Deserialize, Serialize};
use std::str::FromStr;

/// HTTP 包装类型，支持序列化
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HttpMethod(Method);

impl From<Method> for HttpMethod {
    fn from(method: Method) -> Self {
        HttpMethod(method)
    }
}

impl From<HttpMethod> for Method {
    fn from(http_method: HttpMethod) -> Self {
        http_method.0
    }
}

impl Serialize for HttpMethod {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.0.as_str())
    }
}

impl<'de> Deserialize<'de> for HttpMethod {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let method_str: String = Deserialize::deserialize(deserializer)?;
        Method::from_str(&method_str)
            .map(HttpMethod)
            .map_err(|_| serde::de::Error::custom(format!("Invalid HTTP method: {}", method_str)))
    }
}

impl fmt::Display for HttpMethod {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0.as_str())
    }
}

impl HttpMethod {
    pub const GET: HttpMethod = HttpMethod(Method::GET);
    pub const POST: HttpMethod = HttpMethod(Method::POST);
    pub const PUT: HttpMethod = HttpMethod(Method::PUT);
    pub const DELETE: HttpMethod = HttpMethod(Method::DELETE);
    pub const PATCH: HttpMethod = HttpMethod(Method::PATCH);
    pub const HEAD: HttpMethod = HttpMethod(Method::HEAD);
    pub const OPTIONS: HttpMethod = HttpMethod(Method::OPTIONS);
    pub const TRACE: HttpMethod = HttpMethod(Method::TRACE);
    pub const CONNECT: HttpMethod = HttpMethod(Method::CONNECT);

    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }
}
