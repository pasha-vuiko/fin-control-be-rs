use crate::shared::modules::auth::errors::AuthError;
use crate::shared::modules::cache::errors::CacheError;
use aide::OperationIo;
use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use onlyerror::Error;
use schemars::JsonSchema;
use serde::Serialize;
use serde_json::json;
use std::fmt::{Display, Formatter};

#[derive(Serialize, Debug, Error, JsonSchema, OperationIo)]
pub enum HttpError {
    #[error("{0}")]
    NotFound(String),
    #[error("{0}")]
    BadRequest(String),
    #[error("{0}")]
    Forbidden(String),
    #[error("{0}")]
    Unauthorized(String),
    #[error("{0}")]
    Internal(String),
}

impl HttpError {
    pub fn get_name(&self) -> String {
        match self {
            Self::NotFound(_) => "Not Found".into(),
            Self::BadRequest(_) => "Bad Request".into(),
            Self::Unauthorized(_) => "Unauthorized".into(),
            Self::Forbidden(_) => "Forbidden".into(),
            Self::Internal(_) => "Internal Server Error".into(),
        }
    }

    pub fn get_status_code(&self) -> StatusCode {
        match *self {
            Self::NotFound(_) => StatusCode::NOT_FOUND,
            Self::BadRequest(_) => StatusCode::BAD_REQUEST,
            Self::Unauthorized(_) => StatusCode::UNAUTHORIZED,
            Self::Forbidden(_) => StatusCode::FORBIDDEN,
            Self::Internal(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}

impl IntoResponse for HttpError {
    fn into_response(self) -> Response {
        let status = self.get_status_code();
        let error_response = CustomErrorResponse {
            code: status.as_u16(),
            error: self.get_name(),
            message: self.to_string(),
        };

        error_response.into_response()
    }
}

impl From<AuthError> for HttpError {
    fn from(value: AuthError) -> Self {
        match value {
            AuthError::NoAuthHeaderFound(msg) => Self::Unauthorized(msg),
            AuthError::InvalidAuthHeader(msg) => Self::Unauthorized(msg),
            AuthError::InvalidToken(msg) => Self::Unauthorized(msg),
            AuthError::InvalidUserRoles(msg) => Self::Forbidden(msg),
        }
    }
}

impl From<CacheError> for HttpError {
    fn from(value: CacheError) -> Self {
        match value {
            CacheError::KeyNotFound(msg) => Self::Internal(msg),
            CacheError::Unknown(msg) => Self::Internal(msg),
            CacheError::FailedToParseResponse(msg) => Self::Internal(msg),
        }
    }
}

impl From<prisma_client_rust::QueryError> for HttpError {
    fn from(value: prisma_client_rust::QueryError) -> Self {
        Self::Internal(format!("Prisma QueryError: {}", value))
    }
}

impl From<serde_json::Error> for HttpError {
    fn from(value: serde_json::Error) -> Self {
        Self::Internal(format!("Serde JSON Error: {}", value))
    }
}

impl From<Box<dyn std::error::Error>> for HttpError {
    fn from(value: Box<dyn std::error::Error>) -> Self {
        Self::Internal(format!("Error: {}", value))
    }
}

#[derive(Serialize, Debug)]
pub struct CustomErrorResponse {
    pub code: u16,
    pub error: String,
    pub message: String,
}

impl IntoResponse for CustomErrorResponse {
    fn into_response(self) -> Response {
        let body = Json(json!(self));
        let status_code =
            StatusCode::from_u16(self.code).unwrap_or(StatusCode::INTERNAL_SERVER_ERROR);

        (status_code, body).into_response()
    }
}

impl Display for CustomErrorResponse {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}
