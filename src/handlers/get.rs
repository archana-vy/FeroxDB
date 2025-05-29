use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

use crate::{grpc::feroxdb::GetResponse, types::Entry};
use anyhow::Result;

pub async fn handle_get(
    key: String,
    cache: &Arc<Mutex<HashMap<String, Entry>>>,
    _storage: &str,
) -> Result<Option<GetResponse>> {
    let cache = cache.lock().expect("Failed to get cache lock");
    let value = cache.get(&key).and_then(|e| {
        if e.is_expired() {
            Some(GetResponse {
                value: "".to_string(),
                found: false,
            })
        } else {
            Some(GetResponse {
                value: e.value.clone(),
                found: true,
            })
        }
    });

    // if not found in cache read from storage and update cache
    Ok(value)
}
