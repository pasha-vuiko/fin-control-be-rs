use crate::shared::errors::app_error::AppError;
use async_trait::async_trait;
use axum::extract::FromRequestParts;
use axum::http::{request::Parts, StatusCode};
use axum_auth::AuthBearerCustom;

pub struct BearerAuth(pub String);

// this is where you define your custom options
impl AuthBearerCustom for BearerAuth {
    const ERROR_CODE: StatusCode = StatusCode::UNAUTHORIZED; // <-- define custom status code here
    const ERROR_OVERWRITE: Option<&'static str> = Some("test"); // <-- define overwriting message here

    fn from_header(contents: &str) -> Self {
        Self(contents.into())
    }
}

// this is just boilerplate, copy-paste this
#[async_trait]
impl<B> FromRequestParts<B> for BearerAuth
where
    B: Send + Sync,
{
    type Rejection = AppError;

    async fn from_request_parts(parts: &mut Parts, _: &B) -> Result<Self, Self::Rejection> {
        match Self::decode_request_parts(parts) {
            Ok(bearer_token) => Ok(bearer_token),
            Err((_, message)) => {
                let err = AppError::Unauthorized {
                    message: message.into(),
                };

                Err(err)
            }
        }
    }
}
