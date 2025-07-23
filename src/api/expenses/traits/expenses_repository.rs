use async_trait::async_trait;

use crate::api::expenses::dto::create_expense_db_dto::CreateExpenseDbDto;
use crate::api::expenses::dto::find_expenses_dto::FindExpensesDto;
use crate::api::expenses::dto::update_expense_db_dto::UpdateExpenseDbDto;
use crate::api::expenses::types::expense_from_db::ExpenseFromDb;
use crate::shared::errors::http_error::HttpError;

#[async_trait]
pub trait ExpensesRepositoryTrait {
    async fn find_one(&self, id: &str) -> Result<ExpenseFromDb, HttpError>;
    async fn find_many(
        &self,
        filter: Option<FindExpensesDto>,
    ) -> Result<Vec<ExpenseFromDb>, HttpError>;
    async fn create_many(
        &self,
        create_dto: Vec<CreateExpenseDbDto>,
    ) -> Result<Vec<ExpenseFromDb>, HttpError>;
    async fn update_one(
        &self,
        id: &str,
        update_dto: UpdateExpenseDbDto,
    ) -> Result<ExpenseFromDb, HttpError>;
    async fn delete_one(&self, id: &str) -> Result<ExpenseFromDb, HttpError>;
}
