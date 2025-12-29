use chrono::{DateTime, Utc};
use prost_types::Timestamp;
use serde::Serialize;

#[derive(Serialize)]
pub struct ApiResponse<T> {
    pub code: i32,
    pub message: String,
    pub data: Option<T>,
}

// Newtype（Rust 官方推荐模式）
// ProtoTimestamp 是一个 只有一个字段的元组结构体
// 这个字段的索引是 0
// 类型是 Timestamp
pub struct ProtoTimestamp(pub Timestamp);
impl ProtoTimestamp {
    fn into_inner(self) -> Timestamp {
        self.0
    }
}
impl From<DateTime<Utc>> for ProtoTimestamp {
    fn from(dt: DateTime<Utc>) -> Self {
        ProtoTimestamp(Timestamp {
            seconds: dt.timestamp(),
            nanos: dt.timestamp_subsec_nanos() as i32,
        })
    }
}

#[cfg(test)]
mod test {
    use crate::schemas::common::ProtoTimestamp;
    use chrono::Utc;

    #[test]
    pub fn test_proto_transfer() {
        println!(
            "chrono transfer prost timestamp {}",
            // ProtoTimestamp::from(Utc::now()) 返回值类型是:ProtoTimestamp
            // .0 访问 元组结构体的第 0 个字段： imestamp
            ProtoTimestamp::from(Utc::now()).0
        );
        println!(
            "chrono transfer prost timestamp {}",
            // ProtoTimestamp::from(Utc::now()) 返回值类型是:ProtoTimestamp
            // .0 访问 元组结构体的第 0 个字段： imestamp
            ProtoTimestamp::from(Utc::now()).into_inner()
        );
    }
}
