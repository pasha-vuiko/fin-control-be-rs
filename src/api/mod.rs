use aide::axum::routing::get;
use aide::axum::{ApiRouter, IntoApiResponse};
use axum::response::IntoResponse;
use std::{env, sync::Arc};

use crate::shared::errors::http_error::HttpError;
use crate::shared::modules::auth::services::auth0::Auth0Service;
use crate::shared::modules::redis::redis_service::RedisService;
use prisma_client::PrismaClient;

mod customers;
mod expenses;

pub async fn get_router(
    prisma_client: Arc<PrismaClient>,
    redis_service: Arc<RedisService>,
    auth_service: Arc<Auth0Service>,
) -> ApiRouter {
    ApiRouter::new()
        .api_route("/", get(root_handler))
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

async fn root_handler() -> impl IntoApiResponse {
    let response = env::var("CARGO_PKG_VERSION")
        .map(|app_ver| {
            let formatted_response = format!("App version: {app_ver}");

            formatted_response
        })
        .map_err(|err| {
            let err_msg = format!("Failed to get App Version: {err}");

            HttpError::Internal(err_msg)
        });

    response.into_response()
}
