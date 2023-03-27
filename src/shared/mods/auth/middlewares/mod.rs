use axum::body::BoxBody;
use axum::http::Request;
use axum::response::IntoResponse;
use std::sync::Arc;
use tower_http::validate_request::{ValidateRequest, ValidateRequestHeaderLayer};

use crate::shared::errors::app_error::AppError;
use crate::shared::mods::auth::{roles::Roles, service::AuthService, structs::user::User};
use crate::shared::utils::get_bearer_token;

pub struct AuthLayer {
    auth_service: Arc<AuthService>,
}

impl AuthLayer {
    pub fn new(auth_service: Arc<AuthService>) -> Self {
        Self { auth_service }
    }

    pub fn verify(&self, required_roles: Vec<Roles>) -> ValidateRequestHeaderLayer<AuthVerify> {
        let inner_service = AuthVerify::new(self.auth_service.clone(), required_roles);

        ValidateRequestHeaderLayer::custom(inner_service)
    }
}

#[derive(Clone)]
pub struct AuthVerify {
    auth_service: Arc<AuthService>,
    required_roles: Vec<Roles>,
}

impl AuthVerify {
    pub fn new(auth_service: Arc<AuthService>, required_roles: Vec<Roles>) -> Self {
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
        match get_bearer_token(req) {
            Some(token) => {
                let authenticate = self
                    .auth_service
                    .authenticate(&token, self.required_roles.clone());

                match authenticate {
                    Ok(user_claims) => {
                        let user: User = user_claims.into();

                        req.extensions_mut().insert(Some(user));

                        Ok(())
                    }
                    Err(err) => Err(err.into_response()),
                }
            }
            None => {
                let err = AppError::Unauthorized {
                    message: "Missing Authorization header".into(),
                };

                Err(err.into_response())
            }
        }
    }
}
