use aide::axum::IntoApiResponse;
use aide::operation::OperationIo;
use aide::OperationOutput;
use axum::response::IntoResponse;
use axum_jsonschema::JsonSchemaRejection;
use axum_macros::FromRequest;
use serde::Serialize;
use std::fmt::Debug;

use crate::shared::errors::http_error::HttpError;

#[derive(FromRequest, OperationIo)]
#[from_request(via(axum_jsonschema::Json), rejection(HttpError))]
#[aide(
    input_with = "axum_jsonschema::Json<T>",
    output_with = "axum_jsonschema::Json<T>",
    json_schema
)]
pub struct Json<T>(pub T);

impl<T> IntoResponse for Json<T>
where
    T: Serialize,
{
    fn into_response(self) -> axum::response::Response {
        axum::Json(self.0).into_response()
    }
}

impl From<JsonSchemaRejection> for HttpError {
    fn from(rejection: JsonSchemaRejection) -> Self {
        match rejection {
            JsonSchemaRejection::Json(j) => Self::BadRequest(format!("invalid request: {}", j)),
            JsonSchemaRejection::Serde(_) => Self::BadRequest("Serialization error".into()),
            JsonSchemaRejection::Schema(s) => Self::BadRequest(format!("invalid schema: {:?}", s)),
        }
    }
}

impl<T> From<T> for Json<T> {
    fn from(inner: T) -> Self {
        Self(inner)
    }
}

pub fn result_into_json_api_response<T, E>(result: Result<T, E>) -> impl IntoApiResponse
where
    T: Serialize,
    E: Serialize,
{
    result.map(Json).map_err(Json).into_response()
}
