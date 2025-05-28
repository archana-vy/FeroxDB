pub mod cache;
pub mod cli;
pub mod storage;
pub mod types;

use crate::cli::{Cli, Commands};
use cache::Cache;
use clap::Parser;

pub fn run_cli() {
    let args = Cli::parse();
    let mut cache = Cache::new();

    match args.command {
        Commands::Set { key, value, ttl } => {
            println!("Set: {} = {} (ttl: {:?})", key, value, ttl);
            // Call cache + storage logic here

            // cache storage logic
            cache.set(key, value, ttl);

        }
        Commands::Get { key } => {
            println!("Get: {}", key);
            // Lookup logic

            let value = cache.get(key);
            
            println!("Value: {:?}", value.unwrap_or("Not Found".to_string()));
        }
        Commands::Save => {
            println!("Persisting store...");
            // Save logic
        }
    }
}