use crate::grpc::feroxdb::ferox_db_server::{FeroxDb, FeroxDbServer};
use crate::grpc::feroxdb::{
    GetRequest, GetResponse, SaveRequest, SaveResponse, SetRequest, SetResponse,
};
use crate::handlers::{handle_get, handle_save, handle_set};
use crate::types::Entry;
use chrono::Utc;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::time::Duration;
use tokio::signal;
use tonic::{Request, Response, Status, transport::Server};

use crate::server::config::FeroxDbConfig;

#[derive(Clone, Debug, Default)]
pub struct FeroxDbService {
    config: FeroxDbConfig,
    cache: Arc<Mutex<HashMap<String, Entry>>>,
}

impl FeroxDbService {
    fn new(config: FeroxDbConfig) -> Self {
        Self {
            config: config,
            cache: Arc::new(Mutex::new(HashMap::new())),
        }
    }
}

#[tonic::async_trait]
impl FeroxDb for FeroxDbService {
    async fn set(&self, request: Request<SetRequest>) -> Result<Response<SetResponse>, Status> {
        let SetRequest { key, value, ttl } = request.into_inner();

        let value = Entry {
            value,
            expires_at: Utc::now().naive_utc() + Duration::from_secs(ttl),
        };

        let cache = &self.cache;
        let storage = self.config.get_storage_path();

        let res = handle_set(key, value, cache, storage)
            .await
            .expect("Failed to set value");

        Ok(Response::new(res))
    }

    async fn get(&self, request: Request<GetRequest>) -> Result<Response<GetResponse>, Status> {
        let GetRequest { key } = request.into_inner();

        let cache = &self.cache;
        let storage = self.config.get_storage_path();

        let res = handle_get(key, cache, storage)
            .await
            .expect("Failed to get value for key")
            .unwrap_or(GetResponse {
                value: "".to_string(),
                found: false,
            });

        Ok(Response::new(res))
    }

    async fn save(&self, _request: Request<SaveRequest>) -> Result<Response<SaveResponse>, Status> {
        // TODO: save to disk here
        let _storage = self.config.get_storage_path();
        Ok(Response::new(SaveResponse {
            status: "SAVED".to_string(),
        }))
    }
}

pub async fn start_server() -> Result<(), Box<dyn std::error::Error>> {
    let config = FeroxDbConfig::load();
    let app_data = FeroxDbService::new(config);

    let app_data_clone = Arc::new(app_data.clone());

    let addr = format!(
        "{}:{}",
        app_data.config.get_grpc_host(),
        app_data.config.get_grpc_port()
    )
    .parse()?;

    println!("FeroxDB Server running on {}", addr);

    Server::builder()
        .add_service(FeroxDbServer::new(app_data))
        .serve_with_shutdown(addr, async move {
            signal::ctrl_c()
                .await
                .expect("Failed to listen to Ctrl+C....");

            println!("Shuting Down FeroxDB Server now...");

            let cache = &app_data_clone.cache;
            let storage = app_data_clone.config.get_storage_path();

            let _ = handle_save(cache, storage).await.map_err(|err| {
                eprint!("{}", err);
            });
        })
        .await?;

    Ok(())
}
