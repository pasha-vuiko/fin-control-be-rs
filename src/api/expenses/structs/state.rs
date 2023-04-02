use crate::api::expenses::expenses_service::ExpensesService;
use crate::shared::mods::auth::service::AuthService;
use crate::shared::mods::redis::redis_service::RedisService;
use std::sync::Arc;
use axum::extract::FromRef;

#[derive(Clone)]
pub struct ExpensesApiState {
    pub redis_service: Arc<RedisService>,
    pub expenses_service: Arc<ExpensesService>,
    pub auth_service: Arc<AuthService>,
}

impl FromRef<ExpensesApiState> for Arc<ExpensesService> {
    fn from_ref(app_state: &ExpensesApiState) -> Arc<ExpensesService> {
        app_state.expenses_service.clone()
    }
}

impl FromRef<ExpensesApiState> for Arc<RedisService> {
    fn from_ref(app_state: &ExpensesApiState) -> Arc<RedisService> {
        app_state.redis_service.clone()
    }
}

impl FromRef<ExpensesApiState> for Arc<AuthService> {
    fn from_ref(app_state: &ExpensesApiState) -> Arc<AuthService> {
        app_state.auth_service.clone()
    }
}