use crate::grpc::feroxdb::ferox_db_server::{FeroxDb, FeroxDbServer};
use crate::grpc::feroxdb::{
    GetRequest, GetResponse, SaveRequest, SaveResponse, SetRequest, SetResponse,
};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use tonic::{Request, Response, Status, transport::Server};

use crate::server::config::FeroxDbConfig;

#[derive(Debug, Default)]
pub struct FeroxDbService {
    config: FeroxDbConfig,
    cache: Arc<Mutex<HashMap<String, String>>>,
}

#[tonic::async_trait]
impl FeroxDb for FeroxDbService {
    async fn set(&self, request: Request<SetRequest>) -> Result<Response<SetResponse>, Status> {
        let req = request.into_inner();
        let mut map = self.cache.lock().unwrap();
        map.insert(req.key, req.value);
        let _storage = self.config.get_storage_path();
        Ok(Response::new(SetResponse {
            status: "OK".to_string(),
        }))
    }

    async fn get(&self, request: Request<GetRequest>) -> Result<Response<GetResponse>, Status> {
        let req = request.into_inner();
        let map = self.cache.lock().unwrap();
        let _storage = self.config.get_storage_path();
        match map.get(&req.key) {
            Some(value) => Ok(Response::new(GetResponse {
                value: value.clone(),
                found: true,
            })),
            None => Ok(Response::new(GetResponse {
                value: "".to_string(),
                found: false,
            })),
        }
    }

    async fn save(&self, _request: Request<SaveRequest>) -> Result<Response<SaveResponse>, Status> {
        // TODO: save to disk here
        let _storage = self.config.get_storage_path();
        Ok(Response::new(SaveResponse {
            status: "Not Implemented".to_string(),
        }))
    }
}

pub async fn start_server() -> Result<(), Box<dyn std::error::Error>> {
    let config = FeroxDbConfig::load();
    let mut app_data = FeroxDbService::default();

    app_data.config = config;

    let addr = app_data.config.get_grpc_host().to_owned()
        + ":"
        + app_data.config.get_grpc_port().to_string().as_str();
    let addr = addr.parse().expect("Invalid address");

    println!(
        "FeroxDB Server running on {}:{}",
        app_data.config.get_grpc_host(),
        app_data.config.get_grpc_port()
    );

    Server::builder()
        .add_service(FeroxDbServer::new(app_data))
        .serve(addr)
        .await?;

    Ok(())
}
