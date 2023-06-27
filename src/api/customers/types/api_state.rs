use axum::extract::FromRef;
use std::sync::Arc;

use crate::api::customers::customers_service::CustomersService;

#[derive(Clone)]
pub struct CustomersApiState {
    pub customers_service: Arc<CustomersService>,
}

impl FromRef<CustomersApiState> for Arc<CustomersService> {
    fn from_ref(app_state: &CustomersApiState) -> Arc<CustomersService> {
        app_state.customers_service.clone()
    }
}
