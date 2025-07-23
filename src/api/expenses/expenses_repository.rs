use crate::api::expenses::dto::create_expense_db_dto::CreateExpenseDbDto;
use crate::api::expenses::dto::find_expenses_dto::FindExpensesDto;
use crate::api::expenses::dto::update_expense_db_dto::UpdateExpenseDbDto;
use crate::api::expenses::traits::expenses_repository::ExpensesRepositoryTrait;
use crate::api::expenses::types::expense_from_db::ExpenseFromDb;
use crate::shared::errors::http_error::HttpError;
use async_trait::async_trait;
use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, IntoActiveModel, QueryFilter};
use std::sync::Arc;

use crate::shared::modules::db::entities::expense;
use crate::shared::modules::db::entities::prelude::Expense;
use crate::shared::modules::db::entities::expense::ActiveModel as ExpenseActiveModel;

pub struct ExpensesRepository {
    sea_orm_client: Arc<DatabaseConnection>,
}

impl ExpensesRepository {
    pub fn new(sea_orm_client: Arc<DatabaseConnection>) -> Self {
        Self { sea_orm_client }
    }
}

#[async_trait]
impl ExpensesRepositoryTrait for ExpensesRepository {
    async fn find_one(&self, id: &str) -> Result<ExpenseFromDb, HttpError> {
        let found_expense = Expense::find_by_id(id)
            .one(self.sea_orm_client.as_ref())
            .await?
            .ok_or_else(|| HttpError::NotFound(format!("Expense with id {id} not found")))?
            .into();

        Ok(found_expense)
    }

    async fn find_many(
        &self,
        filter: Option<FindExpensesDto>,
    ) -> Result<Vec<ExpenseFromDb>, HttpError> {
        let found_expenses = if let Some(filter) = filter {
            Expense::find()
                .filter(expense::Column::CustomerId.eq(filter.customer_id))
                .all(self.sea_orm_client.as_ref())
                .await?
                .into_iter()
                .map(Into::into)
                .collect()
        } else {
            Expense::find()
                .all(self.sea_orm_client.as_ref())
                .await?
                .into_iter()
                .map(Into::into)
                .collect()
        };

        Ok(found_expenses)
    }

    async fn create_many(
        &self,
        create_dtos: Vec<CreateExpenseDbDto>,
    ) -> Result<Vec<ExpenseFromDb>, HttpError> {
        let created_expenses = Expense::insert_many(
            create_dtos
                .into_iter()
                .map(|create_dto| create_dto.into_active_model()),
        )
        .exec_with_returning_many(self.sea_orm_client.as_ref())
        .await?;

        Ok(created_expenses.into_iter().map(Into::into).collect())
    }

    async fn update_one(
        &self,
        id: &str,
        update_dto: UpdateExpenseDbDto,
    ) -> Result<ExpenseFromDb, HttpError> {
        let updated_expense = Expense::update(ExpenseActiveModel::from(update_dto))
            .filter(expense::Column::Id.eq(id))
            .exec(self.sea_orm_client.as_ref())
            .await?;

        Ok(updated_expense.into())
    }

    async fn delete_one(&self, id: &str) -> Result<ExpenseFromDb, HttpError> {
        let deleted_expense = Expense::delete_by_id(id)
            .exec_with_returning(self.sea_orm_client.as_ref())
            .await?
            .first()
            .ok_or(HttpError::NotFound(format!(
                "Expense with id {id} not found"
            )))?
            .clone()
            .into();

        Ok(deleted_expense)
    }
}
