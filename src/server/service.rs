use crate::grpc::feroxdb::ferox_db_server::FeroxDb;
use crate::grpc::feroxdb::{
    GetRequest, GetResponse, SaveRequest, SaveResponse, SetRequest, SetResponse,
};
use crate::handlers::{handle_get, handle_save, handle_set, load_from_disk};
use crate::types::Entry;
use chrono::Utc;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::time::Duration;
use tonic::{Request, Response, Status};

use crate::server::config::FeroxDbConfig;

#[derive(Clone, Debug, Default)]
pub struct FeroxDbService {
    pub config: FeroxDbConfig,
    pub cache: Arc<Mutex<HashMap<String, Entry>>>,
}

impl FeroxDbService {
    pub fn new(config: FeroxDbConfig) -> Self {
        let cache = load_from_disk(config.get_storage_path())
            .map_err(|err| eprint!("Failed to load cache: {}", err))
            .unwrap_or(HashMap::new());
        
        Self {
            config: config,
            cache: Arc::new(Mutex::new(cache)),
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
        let cache = &self.cache;
        let storage = self.config.get_storage_path();

        let res = handle_save(cache, storage)
            .await
            .expect("Failed to save data");

        Ok(Response::new(res))
    }
}
