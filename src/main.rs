use axum::Router;
use dotenv::dotenv;
use std::{env, net::SocketAddr, sync::Arc};
use tower_request_id::RequestIdLayer;

mod api;
use crate::api::get_root_api_router;
use crate::shared::config::AppConfig;

mod shared;
use crate::shared::config::tracing::{get_tracing_layer, init_tracing};
use crate::shared::handlers::handle_404_resource;
use crate::shared::mods::prisma;
use crate::shared::mods::redis::get_redis_service;

#[tokio::main]
async fn main() {
    // fetch ENV vars from the file if exists
    dotenv().ok();
    // init logging
    init_tracing();
    //config
    let config = envy::from_env::<AppConfig>().expect("failed to parse app config");

    // Prisma client
    let prisma_client = Arc::new(
        prisma::new_client()
            .await
            .expect("Failed to generate prisma client"),
    );
    // Redis Connection manager
    let redis_service = Arc::new(get_redis_service(&config).await);

    // building of an application
    let app = Router::new()
        .merge(get_root_api_router(
            prisma_client,
            redis_service,
            config.clone(),
        ))
        .fallback(handle_404_resource)
        .layer(get_tracing_layer())
        .layer(RequestIdLayer);

    tracing::info!("App version: {}", env::var("CARGO_PKG_VERSION").unwrap());
    tracing::info!(
        "Starting HTTP server: go to http://127.0.0.1:{}",
        config.port
    );

    // Run our application
    let addr = SocketAddr::from(([127, 0, 0, 1], config.port));
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
