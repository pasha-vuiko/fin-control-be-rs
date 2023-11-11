use axum::body::BoxBody;
use axum::http::Request;
use axum::response::IntoResponse;
use std::sync::Arc;
use tower_http::validate_request::{ValidateRequest, ValidateRequestHeaderLayer};

use crate::shared::errors::http_error::HttpError;
use crate::shared::modules::auth::enums::roles::Roles;
use crate::shared::modules::auth::traits::role_based_bearer_auth_service::DynamicAuthService;
use crate::shared::utils::get_bearer_token;

// TODO Possibly replace ValidateRequest implementation with AsyncAuthorizeRequest implementation
pub struct AuthLayer {
    auth_service: Arc<DynamicAuthService>,
}

impl AuthLayer {
    pub fn new(auth_service: Arc<DynamicAuthService>) -> Self {
        Self { auth_service }
    }

    pub fn verify(&self, required_roles: Vec<Roles>) -> ValidateRequestHeaderLayer<AuthVerify> {
        let auth_verifier = AuthVerify::new(self.auth_service.clone(), required_roles);

        ValidateRequestHeaderLayer::custom(auth_verifier)
    }
}

#[derive(Clone)]
pub struct AuthVerify {
    auth_service: Arc<DynamicAuthService>,
    required_roles: Vec<Roles>,
}

impl AuthVerify {
    pub fn new(auth_service: Arc<DynamicAuthService>, required_roles: Vec<Roles>) -> Self {
        Self {
            auth_service,
            required_roles,
        }
    }
}

impl<B> ValidateRequest<B> for AuthVerify {
    type ResponseBody = BoxBody;

    fn validate(
        &mut self,
        req: &mut Request<B>,
    ) -> Result<(), axum::http::Response<Self::ResponseBody>> {
        let Some(token) = get_bearer_token(req) else {
            let err = HttpError::Unauthorized("Missing Authorization header".into());

            return Err(err.into_response());
        };

        let user_result = self
            .auth_service
            .authenticate(&token, self.required_roles.clone());

        match user_result {
            Ok(user) => {
                req.extensions_mut().insert(user);

                Ok(())
            }

            Err(err) => Err(HttpError::from(err).into_response()),
        }
    }
}
