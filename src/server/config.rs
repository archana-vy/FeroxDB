use serde::Deserialize;
use dotenvy::dotenv;

#[derive(Debug, Default, Deserialize)]
pub struct FeroxDbConfig {
    grpc: GrpcConfig,
    storage: StorageConfig,
}

#[derive(Debug, Default, Deserialize)]
struct GrpcConfig {
    host: String,
    port: u16,
}

#[derive(Debug, Default, Deserialize)]
struct StorageConfig {
    path: String,
}

impl FeroxDbConfig {
    pub fn load() -> Self {
        dotenv().ok();
        envy::from_env::<Self>().expect("Failed to load configuration")
    }
    pub fn get_grpc_host(&self) -> &str {
        self.grpc.host.as_str()
    }

    pub fn get_grpc_port(&self) -> u16 {
        self.grpc.port
    }

    pub fn get_storage_path(&self) -> &str {
        self.storage.path.as_str()
    }
}
