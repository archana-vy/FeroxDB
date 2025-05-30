use crate::grpc::feroxdb::ferox_db_server::FeroxDbServer;
use crate::handlers::handle_save;
use crate::server::service::FeroxDbService;
use std::sync::Arc;
use tokio::signal;
use tonic::transport::Server;

use crate::server::config::FeroxDbConfig;

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
