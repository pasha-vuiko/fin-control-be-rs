use axum::routing::get;
use axum::Router;
use std::{env, sync::Arc};

use crate::shared::config::AppConfig;
use crate::shared::errors::app_error::AppError;
use crate::shared::mods::auth::service::AuthService;
use crate::shared::mods::prisma::PrismaClient;
use crate::shared::mods::redis::redis_service::RedisService;

mod customers;

pub async fn get_api_router(
    config: AppConfig,
    prisma_client: Arc<PrismaClient>,
    redis_service: Arc<RedisService>,
    auth_service: Arc<AuthService>,
) -> Router {
    Router::new()
        .route("/", get(index))
        .merge(customers::get_router(
            config,
            prisma_client,
            redis_service,
            auth_service,
        ))
}

async fn index() -> Result<String, AppError> {
    let app_ver = env::var("CARGO_PKG_VERSION");

    match app_ver {
        Ok(app_ver) => Ok(format!("App version: {}", app_ver)),
        Err(err) => Err(AppError::Internal {
            message: format!("Failed to get App Version: {}", err),
        }),
    }
}
