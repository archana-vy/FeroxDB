use crate::{
    client::setup::connect_client,
    grpc::feroxdb::{GetRequest, GetResponse},
};

pub async fn grpc_get(key: &str) -> GetResponse {
    let mut client = connect_client().await;

    let request = tonic::Request::new(GetRequest {
        key: key.to_string(),
    });

    let response = client
        .get(request)
        .await
        .expect("Failed to send grpc get request...")
        .into_inner();

    response
}
