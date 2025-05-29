use dotenvy::dotenv;
use serde::Deserialize;

#[derive(Clone, Debug, Default, Deserialize)]
pub struct FeroxDbConfig {
    grpc: GrpcConfig,
    storage: StorageConfig,
}

#[derive(Clone, Debug, Default, Deserialize)]
struct GrpcConfig {
    host: String,
    port: u16,
}

#[derive(Clone, Debug, Default, Deserialize)]
struct StorageConfig {
    path: String,
}

impl FeroxDbConfig {
    pub fn load() -> Self {
        dotenv().ok();

        let grpc = envy::prefixed("GRPC_")
            .from_env::<GrpcConfig>()
            .expect("Failed to load GRPC config");

        let storage = envy::prefixed("STORAGE_")
            .from_env::<StorageConfig>()
            .expect("Failed to load STORAGE config");

        FeroxDbConfig { grpc, storage }
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
