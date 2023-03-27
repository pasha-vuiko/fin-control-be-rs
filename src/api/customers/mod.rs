use crate::api::customers::customers_repository::CustomerRepository;
use axum::{
    middleware::from_fn_with_state,
    routing::{delete, get, patch, post},
    Router,
};
use std::sync::Arc;

use crate::api::customers::{
    customers_service::CustomersService, structs::state::CustomersApiState,
};
use crate::shared::mods::auth::{middlewares::AuthLayer, roles::Roles, service::AuthService};
use crate::shared::mods::prisma::PrismaClient;
use crate::shared::mods::redis::{middlewares::json_cache, redis_service::RedisService};

mod customers_handlers;
mod customers_repository;
mod customers_service;
mod dto;
mod entities;
mod enums;
mod structs;
mod traits;

pub fn get_router(
    prisma_client: Arc<PrismaClient>,
    redis_service: Arc<RedisService>,
    auth_service: Arc<AuthService>,
) -> Router {
    let customers_repository = Arc::new(CustomerRepository::new(prisma_client));
    let customers_service = CustomersService::new(customers_repository);
    let api_state = CustomersApiState {
        redis_service,
        customers_service,
        auth_service: auth_service.clone(),
    };
    let auth_layer = AuthLayer::new(auth_service);
    let cache_layer = from_fn_with_state(api_state.clone(), json_cache);

    let routes = Router::new()
        // Find one
        .route(
            "/:id",
            get(customers_handlers::find_one)
                .layer(auth_layer.verify(vec![Roles::Admin, Roles::Customer]))
                .layer(cache_layer.clone()),
        )
        // Find many
        .route(
            "/",
            get(customers_handlers::find_many)
                .layer(auth_layer.verify(vec![Roles::Admin]))
                .layer(cache_layer),
        )
        // Create
        .route(
            "/",
            post(customers_handlers::create).layer(auth_layer.verify(vec![Roles::Customer])),
        )
        // Update
        .route(
            "/:id",
            patch(customers_handlers::update)
                .layer(auth_layer.verify(vec![Roles::Admin, Roles::Customer])),
        )
        // Delete
        .route(
            "/:id",
            delete(customers_handlers::delete)
                .layer(auth_layer.verify(vec![Roles::Admin, Roles::Customer])),
        );

    Router::new()
        .nest("/customers", routes)
        .with_state(api_state)
}
