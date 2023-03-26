use axum::extract::{Path, State};
use axum::Json;

use crate::api::customers::{
    dto::{create_customer_dto::CreateCustomerDto, update_customer_dto::UpdateCustomerDto},
    entities::customer_entity::CustomerEntity,
    CustomersApiState,
};
use crate::shared::errors::app_error::AppError;
use crate::shared::mods::auth::{extractors::BearerAuth, roles::Roles, user::User};

pub async fn find_one(
    BearerAuth(token): BearerAuth,
    Path(customer_id): Path<String>,
    State(state): State<CustomersApiState>,
) -> Result<CustomerEntityJson, AppError> {
    // Authorization
    let user: User = state
        .auth_service
        .authenticate(&token, vec![Roles::Admin, Roles::Customer])
        .await?
        .into();

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
    BearerAuth(token): BearerAuth,
    State(state): State<CustomersApiState>,
) -> Result<CustomerEntitiesJson, AppError> {
    // Authorization
    state
        .auth_service
        .authenticate(&token, vec![Roles::Admin])
        .await?;

    let found_products = state.customers_service.find_many().await?;

    Ok(found_products.into())
}

pub async fn create(
    BearerAuth(token): BearerAuth,
    State(state): State<CustomersApiState>,
    Json(create_customer_dto): Json<CreateCustomerDto>,
) -> Result<CustomerEntityJson, AppError> {
    // Authorization
    let user: User = state
        .auth_service
        .authenticate(&token, vec![Roles::Admin, Roles::Customer])
        .await?
        .into();

    let created_customer = state
        .customers_service
        .create(create_customer_dto, &user.id, &user.email)
        .await?;

    Ok(created_customer.into())
}

pub async fn update(
    BearerAuth(token): BearerAuth,
    Path(customer_id): Path<String>,
    State(state): State<CustomersApiState>,
    Json(update_customer_dto): Json<UpdateCustomerDto>,
) -> Result<CustomerEntityJson, AppError> {
    // Authorization
    let user: User = state
        .auth_service
        .authenticate(&token, vec![Roles::Admin, Roles::Customer])
        .await?
        .into();

    if user.roles.contains(&Roles::Admin) {
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
    BearerAuth(token): BearerAuth,
    Path(customer_id): Path<String>,
    State(state): State<CustomersApiState>,
) -> Result<CustomerEntityJson, AppError> {
    // Authorization
    let user: User = state
        .auth_service
        .authenticate(&token, vec![Roles::Admin, Roles::Customer])
        .await?
        .into();

    let deleted_customer = state
        .customers_service
        .delete(&customer_id, &user.id)
        .await?;

    Ok(deleted_customer.into())
}

pub type CustomerEntityJson = Json<CustomerEntity>;
pub type CustomerEntitiesJson = Json<Vec<CustomerEntity>>;
