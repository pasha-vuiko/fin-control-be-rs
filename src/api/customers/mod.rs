use crate::api::customers::customers_repository::CustomerRepository;
use axum::{
    routing::{delete, get, patch, post},
    Router,
};
use std::sync::Arc;

use crate::api::customers::{
    customers_service::CustomersService, structs::state::CustomersApiState,
};
use crate::prisma::PrismaClient;
use crate::shared::mods::auth::enums::roles::Roles;
use crate::shared::mods::auth::middlewares::role_based_bearer_auth::AuthLayer;
use crate::shared::mods::auth::services::auth0::Auth0Service;
use crate::shared::mods::cache::moddlewares::cache::JsonCacheLayer;
use crate::shared::mods::redis::redis_service::RedisService;

mod customers_handlers;
pub mod customers_repository;
pub mod customers_service;
mod dto;
mod entities;
mod enums;
mod structs;
mod traits;

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
