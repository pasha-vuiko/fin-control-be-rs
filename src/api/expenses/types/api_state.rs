use axum::extract::FromRef;
use std::sync::Arc;

use crate::api::expenses::expenses_service::ExpensesService;

#[derive(Clone)]
pub struct ExpensesApiState {
    pub expenses_service: Arc<ExpensesService>,
}

impl FromRef<ExpensesApiState> for Arc<ExpensesService> {
    fn from_ref(app_state: &ExpensesApiState) -> Arc<ExpensesService> {
        app_state.expenses_service.clone()
    }
}
