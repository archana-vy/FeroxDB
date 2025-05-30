use feroxdb::{
    cli::{Cli, Commands},
    client::{grpc_get, grpc_save, grpc_set},
    server,
};

use clap::Parser;

const DEFAULT_TTL_SECONDS: u64 = 60;

#[tokio::main]
pub async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Cli::parse();

    match args.command {
        Commands::Start => {
            server::setup::start_server().await?;
        }
        Commands::Set { key, value, ttl } => {
            println!("Set: {} = {} (ttl: {:?})", key, value, ttl);
            // Call cache + storage logic here

            // cache storage logic
            grpc_set(&key, &value, ttl.unwrap_or(DEFAULT_TTL_SECONDS)).await;
        }
        Commands::Get { key } => {
            println!("Get: {}", key);
            // Lookup logic

            let res = grpc_get(&key).await;

            println!(
                "Value: {:?}",
                if res.found {
                    res.value
                } else {
                    "Not Found".to_string()
                }
            );
        }
        Commands::Save => {
            println!("Persisting store...");
            // Save logic
            let res = grpc_save().await;

            println!("Save response: {:?}", res);
        }
    }

    Ok(())
}
