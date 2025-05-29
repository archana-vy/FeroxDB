use std::{collections::HashMap, sync::{Arc, Mutex}};

use crate::types::Entry;
use anyhow::Result;

pub async fn handle_save(cache: &Arc<Mutex<HashMap<String, Entry>>>, storage: &str) -> Result<()> {
    let cache = cache.lock().unwrap();
    let data = serde_json::to_string(&*cache)?;
    std::fs::write(storage, data)?;
    Ok(())
}
