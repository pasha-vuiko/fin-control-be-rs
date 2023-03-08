use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::Json;
use derive_more::{Display, Error};
use redis::RedisError;
use serde::Serialize;
use serde_json::json;
use std::fmt::{Display, Formatter};

#[derive(Debug, Display, Error)]
pub enum AppError {
    #[display(fmt = message)]
    NotFound { message: String },
    #[display(fmt = message)]
    BadRequest { message: String },
    #[display(fmt = message)]
    Forbidden { message: String },
    #[display(fmt = message)]
    Internal { message: String },
    #[display(fmt = message)]
    Unknown { message: String },
}
impl AppError {
    pub fn get_name(&self) -> String {
        match self {
            Self::NotFound { message: _ } => "Not Found".to_string(),
            Self::BadRequest { message: _ } => "Bad Request".to_string(),
            Self::Forbidden { message: _ } => "Forbidden".to_string(),
            Self::Internal { message: _ } => "Internal Server Error".to_string(),
            Self::Unknown { message: _ } => "Unknown".to_string(),
        }
    }

    fn get_status_code(&self) -> StatusCode {
        match *self {
            Self::NotFound { message: _ } => StatusCode::NOT_FOUND,
            Self::BadRequest { message: _ } => StatusCode::BAD_REQUEST,
            Self::Forbidden { message: _ } => StatusCode::FORBIDDEN,
            Self::Internal { message: _ } => StatusCode::INTERNAL_SERVER_ERROR,
            Self::Unknown { message: _ } => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}
impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let status = self.get_status_code();
        let error_response = CustomErrorResponse {
            code: status.as_u16(),
            error: self.get_name(),
            message: self.to_string(),
        };

        let body = Json(json!(error_response));

        (status, body).into_response()
    }
}
impl From<RedisError> for AppError {
    fn from(source: RedisError) -> Self {
        Self::Internal {
            message: source.to_string(),
        }
    }
}

#[derive(Serialize, Debug)]
pub struct CustomErrorResponse {
    code: u16,
    error: String,
    message: String,
}

impl Display for CustomErrorResponse {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}
