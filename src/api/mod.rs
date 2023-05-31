use axum::routing::get;
use axum::Router;
use std::{env, sync::Arc};

use crate::prisma::PrismaClient;
use crate::shared::errors::app_error::AppError;
use crate::shared::mods::auth::services::auth0::Auth0Service;
use crate::shared::mods::redis::redis_service::RedisService;

mod customers;
mod expenses;

pub async fn get_router(
    prisma_client: Arc<PrismaClient>,
    redis_service: Arc<RedisService>,
    auth_service: Arc<Auth0Service>,
) -> Router {
    Router::new()
        .route("/", get(root_handler))
        .merge(customers::get_router(
            prisma_client.clone(),
            redis_service.clone(),
            auth_service.clone(),
        ))
        .merge(expenses::get_router(
            prisma_client,
            redis_service,
            auth_service,
        ))
}

async fn root_handler() -> Result<String, AppError> {
    env::var("CARGO_PKG_VERSION")
        .map(|app_ver| {
            let formatted_response = format!("App version: {}", app_ver);

            formatted_response
        })
        .map_err(|err| {
            let err_msg = format!("Failed to get App Version: {}", err);

            AppError::Internal(err_msg)
        })
}
