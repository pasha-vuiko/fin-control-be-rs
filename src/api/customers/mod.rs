use crate::api::customers::customers_repository::CustomerRepository;
use axum::{routing::get, Router};
use std::sync::Arc;

use crate::api::customers::customers_service::CustomersService;
use crate::shared::config::AppConfig;
use crate::shared::mods::auth::service::AuthService;
use crate::shared::mods::prisma::PrismaClient;
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
    prisma_client: Arc<PrismaClient>,
    redis_service: Arc<RedisService>,
    config: AppConfig,
    auth_service: AuthService,
) -> Router {
    let customers_repository = Arc::new(CustomerRepository::new(prisma_client));
    let customers_service = CustomersService::new(customers_repository);
    let api_state = CustomersApiState {
        redis_service,
        customers_service,
        config,
        auth_service,
    };

    let routes = Router::new()
        .route(
            "/:id",
            get(customers_handlers::find_one)
                .patch(customers_handlers::update)
                .delete(customers_handlers::delete),
        )
        .route(
            "/",
            get(customers_handlers::find_many).post(customers_handlers::create),
        );

    Router::new()
        .nest("/customers", routes)
        .with_state(api_state)
}

#[derive(Clone)]
pub struct CustomersApiState {
    pub customers_service: CustomersService,
    pub redis_service: Arc<RedisService>,
    pub config: AppConfig,
    pub auth_service: AuthService,
}
