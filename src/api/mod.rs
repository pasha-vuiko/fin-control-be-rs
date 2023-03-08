use axum::routing::get;
use axum::Router;
use std::{env, sync::Arc};

use crate::shared::config::AppConfig;
use crate::shared::mods::prisma::PrismaClient;
use crate::shared::mods::redis::redis_service::RedisService;

mod customers;

pub fn get_root_api_router(
    prisma_client: Arc<PrismaClient>,
    redis_service: Arc<RedisService>,
    config: &AppConfig,
) -> Router {
    Router::new().route("/", get(index))
}

async fn index() -> String {
    let app_ver = env::var("CARGO_PKG_VERSION").unwrap();

    format!("App version: {}", app_ver)
}
