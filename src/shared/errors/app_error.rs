use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use derive_more::{Display, Error};
use prisma_client_rust::QueryError;
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
    Unauthorized { message: String },
    #[display(fmt = message)]
    Internal { message: String },
}
impl AppError {
    pub fn get_name(&self) -> String {
        match self {
            Self::NotFound { message: _ } => "Not Found".into(),
            Self::BadRequest { message: _ } => "Bad Request".into(),
            Self::Unauthorized { message: _ } => "Unauthorized".into(),
            Self::Forbidden { message: _ } => "Forbidden".into(),
            Self::Internal { message: _ } => "Internal Server Error".into(),
        }
    }

    pub fn get_status_code(&self) -> StatusCode {
        match *self {
            Self::NotFound { message: _ } => StatusCode::NOT_FOUND,
            Self::BadRequest { message: _ } => StatusCode::BAD_REQUEST,
            Self::Unauthorized { message: _ } => StatusCode::UNAUTHORIZED,
            Self::Forbidden { message: _ } => StatusCode::FORBIDDEN,
            Self::Internal { message: _ } => StatusCode::INTERNAL_SERVER_ERROR,
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

        error_response.into_response()
    }
}
impl From<RedisError> for AppError {
    fn from(source: RedisError) -> Self {
        Self::Internal {
            message: source.to_string(),
        }
    }
}

impl From<QueryError> for AppError {
    fn from(value: QueryError) -> Self {
        Self::Internal {
            message: format!("Prisma QueryError: {}", value.to_string()),
        }
    }
}

impl From<serde_json::Error> for AppError {
    fn from(value: serde_json::Error) -> Self {
        Self::Internal {
            message: format!("Serde JSON Error: {}", value.to_string()),
        }
    }
}

impl From<alcoholic_jwt::ValidationError> for AppError {
    fn from(value: alcoholic_jwt::ValidationError) -> Self {
        Self::Unauthorized {
            message: format!("JWT Validation Error: {}", value.to_string()),
        }
    }
}

impl From<Box<dyn std::error::Error>> for AppError {
    fn from(value: Box<dyn std::error::Error>) -> Self {
        Self::Internal {
            message: format!("Error: {}", value),
        }
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
