use aide::axum::ApiRouter;
use axum::Extension;
use std::{env, net::SocketAddr, sync::Arc};
use tower_request_id::RequestIdLayer;

mod api;
use crate::shared::config::get_config;

mod shared;
use crate::shared::handlers::handle_404_resource;
use crate::shared::logger;
use crate::shared::modules::auth::services::auth0::Auth0Service;
use crate::shared::modules::open_api::{get_api_docs, get_open_api, get_open_api_router};
use crate::shared::modules::redis::RedisServiceBuilder;

#[tokio::main]
async fn main() {
    let config = get_config().expect("Failed to get config");

    logger::init_logger(&config.log_format, &config.log_level);

    let mut open_api = get_open_api();

    // Prisma client
    let prisma_client = prisma_client::new_client()
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
    let app = ApiRouter::new()
        .merge(api_router)
        .nest_api_service("/docs", get_open_api_router())
        .finish_api_with(&mut open_api, get_api_docs)
        .layer(Extension(Arc::new(open_api)))
        .fallback(handle_404_resource)
        .layer(logger::get_logger_layer())
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
