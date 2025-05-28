use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "fox", version = "0.1", about = "Fast. Persistent. In-memory.")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    Start,
    Set {
        key: String,
        value: String,
        #[arg(long)]
        ttl: Option<u64>,
    },
    Get {
        key: String,
    },
    Save,
}
