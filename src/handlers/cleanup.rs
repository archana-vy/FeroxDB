use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
    time::Duration,
};

use crate::types::Entry;
use tokio::time::sleep;

pub async fn handle_cleanup(cache: Arc<Mutex<HashMap<String, Entry>>>) {
    loop {
        sleep(Duration::from_secs(10)).await;

        let mut map = cache.lock().unwrap();

        map.retain(|_, entry| !entry.is_expired());

        println!("Cleanup ran. Store size: {}", map.len());
    }
}
