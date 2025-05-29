use crate::{client::setup::connect_client, grpc::feroxdb::SetRequest};

pub async fn grpc_set(key: &str, value: &str, ttl: u64) {
    let mut client = connect_client().await;

    let request = tonic::Request::new(SetRequest {
        key: key.to_string(),
        value: value.to_string(),
        ttl,
    });

    let response = client
        .set(request)
        .await
        .expect("Failed to send grpc set request...");
    
    println!("Set response: {:?}", response.into_inner().status);
}
