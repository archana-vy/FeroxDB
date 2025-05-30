use std::{
    collections::HashMap,
    fs,
    path::Path,
    sync::{Arc, Mutex},
};

use crate::{grpc::feroxdb::SaveResponse, types::Entry};
use anyhow::Result;

pub async fn handle_save(
    cache: &Arc<Mutex<HashMap<String, Entry>>>,
    storage: &str,
) -> Result<SaveResponse> {
    let cache = cache.lock().unwrap();
    let data = serde_json::to_string(&*cache)?;

    let path = Path::new(storage);

    // Ensure parent directory exists
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }

    fs::write(storage, data)?;

    Ok(SaveResponse {
        status: "SAVED".to_string(),
    })
}
