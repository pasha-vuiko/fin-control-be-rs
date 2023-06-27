use axum::extract::{Path, State};
use axum::{Extension, Json};
use std::sync::Arc;

use crate::api::customers::customers_service::CustomersService;
use crate::api::customers::{
    dto::{create_customer_dto::CreateCustomerDto, update_customer_dto::UpdateCustomerDto},
    entities::customer_entity::CustomerEntity,
};
use crate::shared::errors::http_error::HttpError;
use crate::shared::mods::auth::structs::user::User;

pub async fn find_one(
    Path(customer_id): Path<String>,
    State(customers_service): State<Arc<CustomersService>>,
) -> Result<CustomerEntityJson, HttpError> {
    let found_customer = customers_service.find_one_by_id(&customer_id).await?;

    Ok(found_customer.into())
}

pub async fn find_one_by_user_id(
    Extension(user): Extension<User>,
    State(customers_service): State<Arc<CustomersService>>,
) -> Result<CustomerEntityJson, HttpError> {
    let found_customer = customers_service.find_one_by_user_id(&user.id).await?;

    Ok(found_customer.into())
}

pub async fn find_many(
    State(customers_service): State<Arc<CustomersService>>,
) -> Result<CustomerEntitiesJson, HttpError> {
    let found_products = customers_service.find_many().await?;

    Ok(found_products.into())
}

pub async fn create(
    Extension(user): Extension<User>,
    State(customers_service): State<Arc<CustomersService>>,
    Json(create_customer_dto): Json<CreateCustomerDto>,
) -> Result<CustomerEntityJson, HttpError> {
    let created_customer = customers_service
        .create(create_customer_dto, &user.id, &user.email)
        .await?;

    Ok(created_customer.into())
}

pub async fn update(
    Extension(user): Extension<User>,
    Path(customer_id): Path<String>,
    State(customers_service): State<Arc<CustomersService>>,
    Json(update_customer_dto): Json<UpdateCustomerDto>,
) -> Result<CustomerEntityJson, HttpError> {
    let updated_customer = if user.is_admin() {
        customers_service
            .update_as_admin(&customer_id, update_customer_dto)
            .await?
    } else {
        customers_service
            .update_as_customer(&customer_id, update_customer_dto, &user.id, &user.email)
            .await?
    };

    Ok(updated_customer.into())
}

pub async fn remove(
    Extension(user): Extension<User>,
    Path(customer_id): Path<String>,
    State(customers_service): State<Arc<CustomersService>>,
) -> Result<CustomerEntityJson, HttpError> {
    let deleted_customer = if user.is_admin() {
        customers_service.delete_as_admin(&customer_id).await?
    } else {
        customers_service
            .delete_as_customer(&customer_id, &user.id)
            .await?
    };

    Ok(deleted_customer.into())
}

pub type CustomerEntityJson = Json<CustomerEntity>;
pub type CustomerEntitiesJson = Json<Vec<CustomerEntity>>;
