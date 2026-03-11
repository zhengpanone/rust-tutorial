use chrono::{DateTime, Utc};

pub struct UserLoggedOut{
    session_id: String,
    occurred_at: DateTime<Utc>,
}