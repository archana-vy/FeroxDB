use feroxdb::{
    server,
    cli::{Cli, Commands},
    cache::Cache,
};

use clap::Parser;

#[tokio::main]
pub async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Cli::parse();
    let mut cache = Cache::new();

    match args.command {
        Commands::Start => {
            server::setup::start_server().await?;
        }
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

    Ok(())
}
