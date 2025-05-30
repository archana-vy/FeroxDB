use crate::{
    client::setup::connect_client,
    grpc::feroxdb::{SaveRequest, SaveResponse},
};

pub async fn grpc_save() -> SaveResponse {
    let mut client = connect_client().await;

    let request = tonic::Request::new(SaveRequest {});

    let response = client
        .save(request)
        .await
        .expect("Failed to send grpc save request...")
        .into_inner();

    response
}
