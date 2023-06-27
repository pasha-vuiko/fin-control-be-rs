use axum::Router;
use dotenv::dotenv;
use std::{env, net::SocketAddr, sync::Arc};
use tower_request_id::RequestIdLayer;

mod prisma;

mod api;
use crate::shared::config::AppConfig;

mod shared;
use crate::shared::config::tracing::{get_tracing_layer, init_tracing};
use crate::shared::handlers::handle_404_resource;
use crate::shared::mods::auth::services::auth0::Auth0Service;
use crate::shared::mods::redis::RedisServiceBuilder;

#[tokio::main]
async fn main() {
    // fetch ENV vars from the file if exists
    dotenv().ok();
    // init logging
    init_tracing();
    //config
    let config = envy::from_env::<AppConfig>().expect("failed to parse app config");

    // Prisma client
    let prisma_client = prisma::new_client()
        .await
        .expect("Failed to generate prisma client");
    // Redis Connection manager
    let redis_service = RedisServiceBuilder::new(&config.redis_host, config.redis_port)
        .with_default_ttl(config.redis_ttl)
        .build()
        .await
        .expect("Failed to generate redis service");
    // Authentication
    let auth_service = Auth0Service::from_auth_domain(&config.auth_auth0_domain)
        .await
        .expect("Failed to generate auth service");

    // TODO Add pagination for APIs
    let api_router = api::get_router(
        Arc::new(prisma_client),
        Arc::new(redis_service),
        Arc::new(auth_service),
    )
    .await;

    // building of an application
    let app = Router::new()
        .merge(api_router)
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
        .expect("Failed to start server");
}
