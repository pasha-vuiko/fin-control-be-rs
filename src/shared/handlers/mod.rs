use axum::response::IntoResponse;

use crate::shared::errors::app_error::AppError;

pub async fn handle_404_resource() -> impl IntoResponse {
    AppError::NotFound {
        message: "The resource was not found".into(),
    }
}
