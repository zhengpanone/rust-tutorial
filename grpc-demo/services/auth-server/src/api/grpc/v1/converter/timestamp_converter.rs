use chrono::{DateTime, TimeZone, Utc};
use prost_types::Timestamp;
/// 时间戳转换器
pub struct TimestampConverter;

impl TimestampConverter {
    /// 将 DateTime 转换为 prost_types::Timestamp
    pub fn to_proto(dt: DateTime<Utc>) -> Timestamp {
        Timestamp {
            seconds: dt.timestamp(),
            nanos: dt.timestamp_subsec_nanos() as i32,
        }
    }

    /// 将 Option<DateTime> 转换为 Option<prost_types::Timestamp>
    pub fn to_proto_opt(dt: Option<DateTime<Utc>>) -> Option<Timestamp> {
        dt.map(Self::to_proto)
    }

    /// 将 prost_types::Timestamp 转换为 DateTime<Utc>
    pub fn to_domain(ts: Timestamp) -> DateTime<Utc> {
        Utc.timestamp_opt(ts.seconds, ts.nanos as u32)
            .single()
            .unwrap_or_else(Utc::now)
    }

    /// 将 Option<prost_types::Timestamp> 转换为 Option<DateTime<Utc>>
    pub fn to_domain_opt(ts: Option<Timestamp>) -> Option<DateTime<Utc>> {
        ts.map(Self::to_domain)
    }
}
