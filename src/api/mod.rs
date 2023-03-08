use std::sync::Arc;
use axum::Router;
use axum::routing::get;
use crate::shared::mods::prisma::PrismaClient;
use crate::shared::mods::redis::redis_service::RedisService;

mod customers;

pub fn get_root_api_router(
    prisma_client: Arc<PrismaClient>,
    redis_service: Arc<RedisService>,
) -> Router {
    Router::new().route("/", get(index))
}

async fn index() -> String {
    let app_ver = "0.1.0";
    // let app_ver = env::var("CARGO_PKG_VERSION").unwrap();

    format!("App version: {}", app_ver)
}