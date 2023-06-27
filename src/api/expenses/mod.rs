use axum::routing::{delete, get, patch, post};
use axum::Router;
use std::sync::Arc;

use crate::api::customers::customers_repository::CustomerRepository;
use crate::api::customers::customers_service::CustomersService;
use crate::api::expenses::expenses_repository::ExpensesRepository;
use crate::api::expenses::expenses_service::ExpensesService;
use crate::api::expenses::types::api_state::ExpensesApiState;

use crate::prisma::PrismaClient;
use crate::shared::mods::auth::enums::roles::Roles;
use crate::shared::mods::auth::middlewares::role_based_bearer_auth::AuthLayer;
use crate::shared::mods::auth::services::auth0::Auth0Service;
use crate::shared::mods::cache::middlewares::json_cache::JsonCacheLayer;
use crate::shared::mods::redis::redis_service::RedisService;

mod dto;
mod entities;
mod traits;
mod types;

mod expenses_handlers;
mod expenses_repository;
mod expenses_service;

pub fn get_router(
    prisma_client: Arc<PrismaClient>,
    redis_service: Arc<RedisService>,
    auth_service: Arc<Auth0Service>,
) -> Router {
    let customers_repository = Arc::new(CustomerRepository::new(prisma_client.clone()));
    let customers_service = Arc::new(CustomersService::new(customers_repository));

    let expenses_repository = Arc::new(ExpensesRepository::new(prisma_client));
    let expenses_service = Arc::new(ExpensesService::new(expenses_repository, customers_service));

    let api_state = ExpensesApiState { expenses_service };
    let auth_layer = AuthLayer::new(auth_service.clone());
    let cache_layer = JsonCacheLayer::new(redis_service, auth_service);

    let routes = Router::new()
        .route(
            "/",
            get(expenses_handlers::find_many)
                .route_layer(cache_layer.clone())
                .route_layer(auth_layer.verify(vec![Roles::Admin, Roles::Customer])),
        )
        .route(
            "/:id",
            get(expenses_handlers::find_one)
                .route_layer(cache_layer)
                .route_layer(auth_layer.verify(vec![Roles::Admin, Roles::Customer])),
        )
        .route(
            "/",
            post(expenses_handlers::create_many)
                .route_layer(auth_layer.verify(vec![Roles::Customer])),
        )
        .route(
            "/:id",
            patch(expenses_handlers::update_one)
                .route_layer(auth_layer.verify(vec![Roles::Customer])),
        )
        .route(
            "/:id",
            delete(expenses_handlers::delete_one)
                .route_layer(auth_layer.verify(vec![Roles::Customer])),
        );

    Router::new()
        .nest("/expenses", routes)
        .with_state(api_state)
}
