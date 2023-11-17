use crate::api::expenses::dto::create_expense_dto::CreateExpenseDto;
use crate::api::expenses::dto::update_expense_dto::UpdateExpenseDto;
use crate::api::expenses::entities::expense_entity::ExpenseEntity;
use crate::api::expenses::expenses_service::ExpensesService;
use crate::shared::errors::http_error::HttpError;
use crate::shared::modules::auth::structs::user::User;
use axum::extract::{Path, State};
use axum::Extension;
use axum_jsonschema::Json;
use std::sync::Arc;

pub async fn find_many(
    Extension(user): Extension<User>,
    State(expenses_service): State<Arc<ExpensesService>>,
) -> Result<ExpenseEntitiesJson, HttpError> {
    let found_expenses = if user.is_admin() {
        expenses_service.find_many().await?
    } else {
        expenses_service.find_many_as_customer(&user.id).await?
    };

    Ok(Json(found_expenses))
}

pub async fn find_one(
    Path(expense_id): Path<String>,
    Extension(user): Extension<User>,
    State(expenses_service): State<Arc<ExpensesService>>,
) -> Result<ExpenseEntityJson, HttpError> {
    let found_expense = if user.is_admin() {
        expenses_service.find_one_as_admin(&expense_id).await?
    } else {
        expenses_service
            .find_one_as_customer(&expense_id, &user.id)
            .await?
    };

    Ok(Json(found_expense))
}

pub async fn create_many(
    Extension(user): Extension<User>,
    State(expenses_service): State<Arc<ExpensesService>>,
    Json(expense_entities): Json<Vec<CreateExpenseDto>>,
) -> Result<ExpenseEntitiesJson, HttpError> {
    let created_expenses = expenses_service
        .create_many(expense_entities, &user.id)
        .await?;

    Ok(Json(created_expenses))
}

pub async fn update_one(
    Path(expense_id): Path<String>,
    Extension(user): Extension<User>,
    State(expenses_service): State<Arc<ExpensesService>>,
    Json(update_dto): Json<UpdateExpenseDto>,
) -> Result<ExpenseEntityJson, HttpError> {
    let updated_expense = expenses_service
        .update(&expense_id, update_dto, &user.id)
        .await?;

    Ok(Json(updated_expense))
}

pub async fn delete_one(
    Path(expense_id): Path<String>,
    Extension(user): Extension<User>,
    State(expenses_service): State<Arc<ExpensesService>>,
) -> Result<ExpenseEntityJson, HttpError> {
    let deleted_expense = expenses_service.delete(&expense_id, &user.id).await?;

    Ok(Json(deleted_expense))
}

pub type ExpenseEntityJson = Json<ExpenseEntity>;
pub type ExpenseEntitiesJson = Json<Vec<ExpenseEntity>>;
