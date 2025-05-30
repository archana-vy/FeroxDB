use std::time::Duration;

use chrono::{NaiveDateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct Entry {
    pub value: String,
    pub expires_at: NaiveDateTime,
}

impl Entry {
    pub fn new(value: String, ttl: u64) -> Self {
        let expires_at = Utc::now().naive_utc() + Duration::from_secs(ttl);
        Self { value, expires_at }
    }

    pub fn is_expired(&self) -> bool {
        self.expires_at < Utc::now().naive_utc()
    }
}
