use std::{collections::HashMap, fs};

use crate::types::Entry;

pub fn load_from_disk(storage: &str) -> std::io::Result<HashMap<String, Entry>> {
    let data = fs::read_to_string(storage)?;
    let map: HashMap<String, Entry> = serde_json::from_str(&data)?;
    Ok(map)
}
