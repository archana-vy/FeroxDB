use std::time::{Duration, Instant};

pub struct Entry {
    pub value: String,
    pub expires_at: Instant,
}

impl Entry {
    pub fn new(value: String, ttl: u64) -> Self {
        let expires_at = Instant::now() + Duration::from_secs(ttl);
        Self { value, expires_at }
    }

    pub fn is_expired(&self) -> bool {
        self.expires_at < Instant::now()
    }
}
