use crate::shared::errors::app_error::AppError;
use axum::response::IntoResponse;

pub async fn handle_404_resource() -> impl IntoResponse {
    AppError::NotFound {
        message: "The resource was not found".to_string(),
    }
}
