use crate::api::customers::customers_repository::CustomerRepository;
use axum::{
    routing::{delete, get, patch, post},
    Router,
};
use std::sync::Arc;

use crate::api::customers::{
    customers_service::CustomersService, types::api_state::CustomersApiState,
};
use crate::shared::modules::auth::enums::roles::Roles;
use crate::shared::modules::auth::middlewares::role_based_bearer_auth::AuthLayer;
use crate::shared::modules::auth::services::auth0::Auth0Service;
use crate::shared::modules::cache::middlewares::json_cache::JsonCacheLayer;
use crate::shared::modules::redis::redis_service::RedisService;
use prisma_client::PrismaClient;

mod customers_handlers;
pub mod customers_repository;
pub mod customers_service;
mod dto;
mod entities;
mod traits;
mod types;

pub fn get_router(
    prisma_client: Arc<PrismaClient>,
    redis_service: Arc<RedisService>,
    auth_service: Arc<Auth0Service>,
) -> Router {
    let customers_repository = Arc::new(CustomerRepository::new(prisma_client));
    let customers_service = Arc::new(CustomersService::new(customers_repository));
    let api_state = CustomersApiState { customers_service };

    let auth_layer = AuthLayer::new(auth_service.clone());
    let cache_layer = JsonCacheLayer::new(redis_service, auth_service);

    let routes = Router::new()
        .route(
            "/self",
            get(customers_handlers::find_one_by_user_id)
                .route_layer(cache_layer.clone())
                .route_layer(auth_layer.verify(vec![Roles::Customer])),
        )
        .route(
            "/:id",
            get(customers_handlers::find_one).route_layer(auth_layer.verify(vec![Roles::Admin])),
        )
        .route(
            "/",
            get(customers_handlers::find_many)
                .route_layer(cache_layer)
                .route_layer(auth_layer.verify(vec![Roles::Admin])),
        )
        .route(
            "/",
            post(customers_handlers::create).route_layer(auth_layer.verify(vec![Roles::Customer])),
        )
        .route(
            "/:id",
            patch(customers_handlers::update)
                .route_layer(auth_layer.verify(vec![Roles::Admin, Roles::Customer])),
        )
        .route(
            "/:id",
            delete(customers_handlers::remove)
                .route_layer(auth_layer.verify(vec![Roles::Admin, Roles::Customer])),
        );

    Router::new()
        .nest("/customers", routes)
        .with_state(api_state)
}
