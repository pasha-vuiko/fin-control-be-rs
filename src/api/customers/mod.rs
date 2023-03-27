use crate::api::customers::customers_repository::CustomerRepository;
use axum::{middleware::from_fn_with_state, routing::get, Router};
use std::sync::Arc;

use crate::api::customers::customers_service::CustomersService;
use crate::api::customers::structs::state::CustomersApiState;
use crate::shared::config::AppConfig;
use crate::shared::mods::auth::middlewares::AuthLayer;
use crate::shared::mods::auth::roles::Roles;
use crate::shared::mods::auth::service::AuthService;
use crate::shared::mods::prisma::PrismaClient;
use crate::shared::mods::redis::middlewares::json_cache;
use crate::shared::mods::redis::redis_service::RedisService;

mod customers_handlers;
mod customers_repository;
mod customers_service;
mod dto;
mod entities;
mod enums;
mod structs;
mod traits;

pub fn get_router(
    config: AppConfig,
    prisma_client: Arc<PrismaClient>,
    redis_service: Arc<RedisService>,
    auth_service: Arc<AuthService>,
) -> Router {
    let customers_repository = Arc::new(CustomerRepository::new(prisma_client));
    let customers_service = CustomersService::new(customers_repository);
    let api_state = CustomersApiState {
        config,
        redis_service,
        customers_service,
        auth_service: auth_service.clone(),
    };
    let auth_layer = AuthLayer::new(auth_service);
    let cache_layer = from_fn_with_state(api_state.clone(), json_cache);

    let routes = Router::new()
        .route(
            "/:id",
            // Find one
            get(customers_handlers::find_one)
                .layer(auth_layer.verify(vec![Roles::Admin, Roles::Customer]))
                .layer(cache_layer.clone())
                // Update
                .patch(customers_handlers::update)
                .layer(auth_layer.verify(vec![Roles::Admin, Roles::Customer]))
                // Delete
                .delete(customers_handlers::delete)
                .layer(auth_layer.verify(vec![Roles::Admin, Roles::Customer])),
        )
        .route(
            "/",
            // Find many
            get(customers_handlers::find_many)
                .layer(auth_layer.verify(vec![Roles::Admin]))
                .layer(cache_layer)
                // Create
                .post(customers_handlers::create)
                .layer(auth_layer.verify(vec![Roles::Customer])),
        );

    Router::new()
        .nest("/customers", routes)
        .with_state(api_state)
}
