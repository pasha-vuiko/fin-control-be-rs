use aide::axum::ApiRouter;
use axum::Extension;
use std::{env, sync::Arc};
use sea_orm::Database;

mod api;
use crate::shared::config::get_config;

mod shared;
use crate::shared::handlers::handle_404_resource;
use crate::shared::modules::auth::services::auth0::Auth0Service;
use crate::shared::modules::logger;
use crate::shared::modules::logger::middlewares::get_request_id_layer;
use crate::shared::modules::open_api::{get_api_docs, get_open_api, get_open_api_router};
use crate::shared::modules::redis::RedisServiceBuilder;

#[tokio::main]
async fn main() {
    let config = get_config().expect("Failed to get config");

    logger::init_logger(&config.log_format, &config.log_level);

    let mut open_api = get_open_api();

    // SeaORM client
    let sea_orm = Database::connect(&config.database_url)
        .await
        .expect("Failed to connect to DB");
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
        Arc::new(sea_orm),
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
        .layer(get_request_id_layer());

    let app_version = env::var("CARGO_PKG_VERSION").expect("Failed to get app version");
    let port = config.port;

    tracing::info!("App version: {app_version}");
    tracing::info!("Starting HTTP server: go to http://127.0.0.1:{port}",);

    // Run our application
    let addr = format!("127.0.0.1:{port}");
    let listener = tokio::net::TcpListener::bind(addr)
        .await
        .expect("Failed to bind port");

    axum::serve(listener, app.into_make_service())
        .await
        .expect("Failed to start server");
}
