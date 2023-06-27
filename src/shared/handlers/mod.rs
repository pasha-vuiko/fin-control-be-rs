use axum::response::IntoResponse;

use crate::shared::errors::http_error::HttpError;

pub async fn handle_404_resource() -> impl IntoResponse {
    HttpError::NotFound("The resource was not found".into())
}
