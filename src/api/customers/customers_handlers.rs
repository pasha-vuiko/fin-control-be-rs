use crate::api::customers::dto::create_customer_dto::CreateCustomerDto;
use crate::api::customers::dto::update_customer_dto::UpdateCustomerDto;
use crate::api::customers::entities::customer_entity::CustomerEntity;
use crate::api::customers::CustomersApiState;
use crate::shared::errors::app_error::AppError;
use crate::shared::mods::auth::user::User;
use crate::shared::mods::auth::{extractors::BearerAuth, roles::Roles};
use axum::extract::{OriginalUri, Path, State};
use axum::Json;

pub async fn find_one(
    BearerAuth(token): BearerAuth,
    Path(customer_id): Path<String>,
    OriginalUri(original_uri): OriginalUri,
    State(state): State<CustomersApiState>,
) -> Result<CustomerEntityJson, AppError> {
    // Authorization
    let user_claims = state
        .auth_service
        .authenticate(&token, vec![Roles::Admin, Roles::Customer])
        .await?;
    let user = User::from(user_claims);

    let req_uri = original_uri.to_string();

    if user.roles.contains(&Roles::Customer) {
        let found_customer = state
            .redis_service
            .wrap(
                || {
                    state
                        .customers_service
                        .find_one_as_customer(&customer_id, &user.id)
                },
                &req_uri,
                state.config.redis_ttl,
            )
            .await?;

        return Ok(found_customer.into());
    }

    let found_customer = state
        .customers_service
        .find_one_as_admin(&customer_id)
        .await?;

    Ok(found_customer.into())
}

pub async fn find_many(
    OriginalUri(original_uri): OriginalUri,
    BearerAuth(token): BearerAuth,
    State(state): State<CustomersApiState>,
) -> Result<CustomerEntitiesJson, AppError> {
    // Authorization
    state
        .auth_service
        .authenticate(&token, vec![Roles::Admin])
        .await?;

    let req_uri = original_uri.to_string();
    let found_products = state
        .redis_service
        .wrap(
            || state.customers_service.find_many(),
            &req_uri,
            state.config.redis_ttl,
        )
        .await?;

    Ok(found_products.into())
}

pub async fn create(
    BearerAuth(token): BearerAuth,
    State(state): State<CustomersApiState>,
    Json(create_customer_dto): Json<CreateCustomerDto>,
) -> Result<CustomerEntityJson, AppError> {
    // Authorization
    let user_claims = state
        .auth_service
        .authenticate(&token, vec![Roles::Customer])
        .await?;
    let user = User::from(user_claims);

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
    let user_claims = state
        .auth_service
        .authenticate(&token, vec![Roles::Admin, Roles::Customer])
        .await?;
    let user = User::from(user_claims);

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
    let user_claims = state
        .auth_service
        .authenticate(&token, vec![Roles::Admin, Roles::Customer])
        .await?;
    let user = User::from(user_claims);

    let deleted_customer = state
        .customers_service
        .delete(&customer_id, &user.id)
        .await?;

    Ok(deleted_customer.into())
}

pub type CustomerEntityJson = Json<CustomerEntity>;
pub type CustomerEntitiesJson = Json<Vec<CustomerEntity>>;
