pub mod cache;
pub mod cli;
pub mod storage;
pub mod types;

use crate::cli::{Cli, Commands};
use clap::Parser;

pub fn run_cli() {
    let args = Cli::parse();

    match args.command {
        Commands::Set { key, value, ttl } => {
            println!("Set: {} = {} (ttl: {:?})", key, value, ttl);
            // Call cache + storage logic here
        }
        Commands::Get { key } => {
            println!("Get: {}", key);
            // Lookup logic
        }
        Commands::Save => {
            println!("Persisting store...");
            // Save logic
        }
    }
}