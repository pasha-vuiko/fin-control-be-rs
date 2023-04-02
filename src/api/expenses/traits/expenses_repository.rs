use crate::api::expenses::dto::create_expense_db_dto::CreateExpenseDbDto;
use crate::api::expenses::dto::find_expenses_dto::FindExpensesDto;
use crate::api::expenses::dto::update_expense_db_dto::UpdateExpenseDbDto;
use async_trait::async_trait;

use crate::api::expenses::structs::expense_from_db::ExpenseFromDb;
use crate::shared::errors::app_error::AppError;

#[async_trait]
pub trait ExpensesRepositoryTrait {
    async fn find_one(&self, id: &str) -> Result<ExpenseFromDb, AppError>;
    async fn find_many(
        &self,
        filter: Option<FindExpensesDto>,
    ) -> Result<Vec<ExpenseFromDb>, AppError>;
    async fn create_many(
        &self,
        create_dto: Vec<CreateExpenseDbDto>,
        customer_id: &str,
    ) -> Result<Vec<ExpenseFromDb>, AppError>;
    async fn update_one(
        &self,
        id: &str,
        update_dto: UpdateExpenseDbDto,
    ) -> Result<ExpenseFromDb, AppError>;
    async fn delete_one(&self, id: &str) -> Result<ExpenseFromDb, AppError>;
}
