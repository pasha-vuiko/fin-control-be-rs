use crate::api::customers::customers_service::CustomersService;
use crate::shared::config::AppConfig;
use crate::shared::mods::auth::service::AuthService;
use crate::shared::mods::redis::redis_service::RedisService;
use axum::extract::FromRef;
use std::sync::Arc;

#[derive(Clone)]
pub struct CustomersApiState {
    pub config: AppConfig,
    pub customers_service: CustomersService,
    pub redis_service: Arc<RedisService>,
    pub auth_service: Arc<AuthService>,
}

impl FromRef<CustomersApiState> for Arc<RedisService> {
    fn from_ref(app_state: &CustomersApiState) -> Arc<RedisService> {
        app_state.redis_service.clone()
    }
}

impl FromRef<CustomersApiState> for Arc<AuthService> {
    fn from_ref(app_state: &CustomersApiState) -> Arc<AuthService> {
        app_state.auth_service.clone()
    }
}
