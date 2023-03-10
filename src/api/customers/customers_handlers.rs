use crate::api::customers::dto::create_customer_dto::CreateCustomerDto;
use crate::api::customers::dto::update_customer_dto::UpdateCustomerDto;
use crate::api::customers::entities::customer_entity::CustomerEntity;
use crate::api::customers::CustomersApiState;
use crate::shared::errors::app_error::AppError;
use axum::extract::{OriginalUri, Path, State};
use axum::Json;

// TODO Add Authorization with auth0

pub async fn find_one(
    Path(customer_id): Path<String>,
    OriginalUri(original_uri): OriginalUri,
    State(api_state): State<CustomersApiState>,
) -> Result<CustomerEntityJson, AppError> {
    let req_uri = original_uri.to_string();

    let found_product = api_state
        .redis_service
        .wrap(
            || api_state.customers_service.find_one(&customer_id),
            &req_uri,
            api_state.config.redis_ttl,
        )
        .await?;

    Ok(found_product.into())
}

pub async fn find_many(
    OriginalUri(original_uri): OriginalUri,
    State(api_state): State<CustomersApiState>,
) -> Result<CustomerEntitiesJson, AppError> {
    let req_uri = original_uri.to_string();

    let found_products = api_state
        .redis_service
        .wrap(
            || api_state.customers_service.find_many(),
            &req_uri,
            api_state.config.redis_ttl,
        )
        .await?;

    Ok(found_products.into())
}

pub async fn create(
    State(api_state): State<CustomersApiState>,
    Json(create_customer_dto): Json<CreateCustomerDto>,
) -> Result<CustomerEntityJson, AppError> {
    let created_customer = api_state
        .customers_service
        .create(create_customer_dto)
        .await?;

    Ok(created_customer.into())
}

pub async fn update(
    Path(customer_id): Path<String>,
    State(api_state): State<CustomersApiState>,
    Json(update_customer_dto): Json<UpdateCustomerDto>,
) -> Result<CustomerEntityJson, AppError> {
    let updated_customer = api_state
        .customers_service
        .update(&customer_id, update_customer_dto)
        .await?;

    Ok(updated_customer.into())
}

pub async fn delete(
    Path(customer_id): Path<String>,
    State(api_state): State<CustomersApiState>,
) -> Result<CustomerEntityJson, AppError> {
    let deleted_customer = api_state.customers_service.delete(&customer_id).await?;

    Ok(deleted_customer.into())
}

pub type CustomerEntityJson = Json<CustomerEntity>;
pub type CustomerEntitiesJson = Json<Vec<CustomerEntity>>;
