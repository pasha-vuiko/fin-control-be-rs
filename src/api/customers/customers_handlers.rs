use crate::api::customers::dto::create_customer_dto::CreateCustomerDto;
use crate::api::customers::dto::update_customer_dto::UpdateCustomerDto;
use crate::api::customers::entities::customer_entity::CustomerEntity;
use crate::api::customers::CustomersApiState;
use crate::shared::errors::app_error::AppError;
use crate::shared::mods::auth::{extractors::BearerAuth, roles::Roles};
use axum::extract::{OriginalUri, Path, State};
use axum::Json;

pub async fn find_one(
    BearerAuth(token): BearerAuth,
    Path(customer_id): Path<String>,
    OriginalUri(original_uri): OriginalUri,
    State(api_state): State<CustomersApiState>,
) -> Result<CustomerEntityJson, AppError> {
    // Authorization
    api_state
        .auth_service
        .authenticate(&token, vec![Roles::Admin, Roles::Customer])
        .await?;

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
    BearerAuth(token): BearerAuth,
    State(api_state): State<CustomersApiState>,
) -> Result<CustomerEntitiesJson, AppError> {
    // Authorization
    api_state
        .auth_service
        .authenticate(&token, vec![Roles::Admin])
        .await?;

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
    BearerAuth(token): BearerAuth,
    State(api_state): State<CustomersApiState>,
    Json(create_customer_dto): Json<CreateCustomerDto>,
) -> Result<CustomerEntityJson, AppError> {
    // Authorization
    api_state
        .auth_service
        .authenticate(&token, vec![Roles::Customer])
        .await?;

    let created_customer = api_state
        .customers_service
        .create(create_customer_dto)
        .await?;

    Ok(created_customer.into())
}

pub async fn update(
    BearerAuth(token): BearerAuth,
    Path(customer_id): Path<String>,
    State(api_state): State<CustomersApiState>,
    Json(update_customer_dto): Json<UpdateCustomerDto>,
) -> Result<CustomerEntityJson, AppError> {
    // Authorization
    api_state
        .auth_service
        .authenticate(&token, vec![Roles::Admin, Roles::Customer])
        .await?;

    let updated_customer = api_state
        .customers_service
        .update(&customer_id, update_customer_dto)
        .await?;

    Ok(updated_customer.into())
}

pub async fn delete(
    BearerAuth(token): BearerAuth,
    Path(customer_id): Path<String>,
    State(api_state): State<CustomersApiState>,
) -> Result<CustomerEntityJson, AppError> {
    // Authorization
    api_state
        .auth_service
        .authenticate(&token, vec![Roles::Admin, Roles::Customer])
        .await?;

    let deleted_customer = api_state.customers_service.delete(&customer_id).await?;

    Ok(deleted_customer.into())
}

pub type CustomerEntityJson = Json<CustomerEntity>;
pub type CustomerEntitiesJson = Json<Vec<CustomerEntity>>;
