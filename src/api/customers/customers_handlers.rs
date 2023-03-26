use axum::extract::{Path, State};
use axum::Json;

use crate::api::customers::{
    dto::{create_customer_dto::CreateCustomerDto, update_customer_dto::UpdateCustomerDto},
    entities::customer_entity::CustomerEntity,
    CustomersApiState,
};
use crate::shared::errors::app_error::AppError;
use crate::shared::mods::auth::service::AuthService;
use crate::shared::mods::auth::{extractors::BearerAuth, roles::Roles};

pub async fn find_one(
    BearerAuth(user): BearerAuth,
    Path(customer_id): Path<String>,
    State(state): State<CustomersApiState>,
) -> Result<CustomerEntityJson, AppError> {
    // Authorization
    AuthService::check_user_roles(&[Roles::Admin, Roles::Customer], &user)?;

    if user.is_admin() {
        let found_customer = state
            .customers_service
            .find_one_as_admin(&customer_id)
            .await?;

        return Ok(found_customer.into());
    }

    let found_customer = state
        .customers_service
        .find_one_as_customer(&customer_id, &user.id)
        .await?;

    Ok(found_customer.into())
}

pub async fn find_many(
    BearerAuth(user): BearerAuth,
    State(state): State<CustomersApiState>,
) -> Result<CustomerEntitiesJson, AppError> {
    // Authorization
    AuthService::check_user_roles(&[Roles::Admin], &user)?;

    let found_products = state.customers_service.find_many().await?;

    Ok(found_products.into())
}

pub async fn create(
    BearerAuth(user): BearerAuth,
    State(state): State<CustomersApiState>,
    Json(create_customer_dto): Json<CreateCustomerDto>,
) -> Result<CustomerEntityJson, AppError> {
    // Authorization
    AuthService::check_user_roles(&[Roles::Admin, Roles::Customer], &user)?;

    let created_customer = state
        .customers_service
        .create(create_customer_dto, &user.id, &user.email)
        .await?;

    Ok(created_customer.into())
}

pub async fn update(
    BearerAuth(user): BearerAuth,
    Path(customer_id): Path<String>,
    State(state): State<CustomersApiState>,
    Json(update_customer_dto): Json<UpdateCustomerDto>,
) -> Result<CustomerEntityJson, AppError> {
    // Authorization
    AuthService::check_user_roles(&[Roles::Admin, Roles::Customer], &user)?;

    if user.is_admin() {
        let updated_customer = state
            .customers_service
            .update_as_admin(&customer_id, update_customer_dto)
            .await?;

        return Ok(updated_customer.into());
    }

    let updated_customer = state
        .customers_service
        .update_as_customer(&customer_id, update_customer_dto, &user.id)
        .await?;

    Ok(updated_customer.into())
}

pub async fn delete(
    BearerAuth(user): BearerAuth,
    Path(customer_id): Path<String>,
    State(state): State<CustomersApiState>,
) -> Result<CustomerEntityJson, AppError> {
    // Authorization
    AuthService::check_user_roles(&[Roles::Admin, Roles::Customer], &user)?;

    let deleted_customer = state
        .customers_service
        .delete(&customer_id, &user.id)
        .await?;

    Ok(deleted_customer.into())
}

pub type CustomerEntityJson = Json<CustomerEntity>;
pub type CustomerEntitiesJson = Json<Vec<CustomerEntity>>;
