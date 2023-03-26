use async_trait::async_trait;
use axum::extract::{FromRequestParts, State};
use axum::http::{header, request::Parts};
use std::convert::Infallible;
use std::sync::Arc;

use crate::shared::errors::app_error::AppError;
use crate::shared::mods::auth::service::AuthService;
use crate::shared::mods::auth::user::User;

pub struct BearerAuth(pub User);

// this is just boilerplate, copy-paste this
#[async_trait]
impl<S> FromRequestParts<S> for BearerAuth
where
    S: Send + Sync,
    Arc<AuthService>: axum::extract::FromRef<S>,
{
    type Rejection = AppError;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        match get_bearer_token(parts) {
            Some(token) => {
                let state: Result<State<Arc<AuthService>>, Infallible> =
                    State::from_request_parts(parts, state).await;

                match state {
                    Ok(state) => {
                        let State(auth_service) = state;

                        let user_claims = auth_service.authenticate(&token, vec![]).await?;

                        Ok(BearerAuth(user_claims.into()))
                    }
                    Err(err) => Err(AppError::Internal {
                        message: err.to_string(),
                    }),
                }
            }
            None => Err(AppError::Unauthorized {
                message: "Missing Authorization header".into(),
            }),
        }
    }
}

fn get_bearer_token(parts: &Parts) -> Option<String> {
    let authorization_header = parts.headers.get(header::AUTHORIZATION)?;
    let bearer_token = authorization_header.to_str().ok()?;
    let bearer_token = bearer_token.trim_start_matches("Bearer ");

    Some(bearer_token.to_string())
}
