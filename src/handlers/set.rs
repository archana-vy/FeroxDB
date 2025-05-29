use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

use crate::{grpc::feroxdb::SetResponse, types::Entry};
use anyhow::Result;

pub async fn handle_set(
    key: String,
    value: Entry,
    cache: &Arc<Mutex<HashMap<String, Entry>>>,
    _storage: &str, // not storing to disk at the moment
) -> Result<SetResponse> {
    let mut cache = cache.lock().expect("Failed to get cache lock");
    cache.insert(key, value);
    Ok(SetResponse {
        status: "OK".to_string(),
    })
}
