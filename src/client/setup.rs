use std::env;

use crate::grpc::feroxdb::ferox_db_client::FeroxDbClient;
use dotenvy::dotenv;
use tonic::transport::Channel;

pub async fn connect_client() -> FeroxDbClient<Channel> {
    FeroxDbClient::connect(get_grpc_url())
        .await
        .expect("Failed to connect to grpc server...")
}

pub fn get_grpc_url() -> String {
    dotenv().ok(); // Load .env if available

    let host = env::var("GRPC_HOST").unwrap_or_else(|_| "127.0.0.1".to_string());
    let port = env::var("GRPC_PORT").unwrap_or_else(|_| "50051".to_string());

    format!("http://{}:{}", host, port)
}
