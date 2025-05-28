use std::collections::HashMap;

use crate::types::Entry;

const DEFAULT_TTL_SECONDS: u64 = 60;

pub struct Cache {
    pub store: HashMap<String, Entry>,
}

impl Cache {
    pub fn new() -> Self {
        Self {
            store: HashMap::new(),
        }
    }

    pub fn set(&mut self, key: String, value: String, ttl: Option<u64>) {
        let ttl = ttl.unwrap_or(DEFAULT_TTL_SECONDS);
        self.store.insert(key, Entry::new(value, ttl));
    }

    pub fn get(&self, key: String) -> Option<String> {
        self.store.get(&key).and_then(|entry| {
            if entry.is_expired() {
                return None;
            } else {
                return Some(entry.value.clone());
            }
        })
    }

    pub fn all(&self) -> &HashMap<String, Entry> {
        &self.store
    }
}
