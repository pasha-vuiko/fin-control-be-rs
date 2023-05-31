use async_trait::async_trait;
use prisma_client_rust::Direction;
use std::sync::Arc;

use crate::api::expenses::dto::create_expense_db_dto::CreateExpenseDbDto;
use crate::api::expenses::dto::find_expenses_dto::FindExpensesDto;
use crate::api::expenses::dto::update_expense_db_dto::UpdateExpenseDbDto;
use crate::api::expenses::structs::expense_from_db::ExpenseFromDb;
use crate::api::expenses::traits::expenses_repository::ExpensesRepositoryTrait;
use crate::prisma::{expense, PrismaClient};
use crate::shared::errors::app_error::AppError;

pub struct ExpensesRepository {
    prisma_client: Arc<PrismaClient>,
}

impl ExpensesRepository {
    pub fn new(prisma_client: Arc<PrismaClient>) -> Self {
        Self { prisma_client }
    }
}

#[async_trait]
impl ExpensesRepositoryTrait for ExpensesRepository {
    async fn find_one(&self, id: &str) -> Result<ExpenseFromDb, AppError> {
        let found_expense = self
            .prisma_client
            .expense()
            .find_unique(expense::id::equals(id.into()))
            .exec()
            .await?
            .ok_or_else(|| AppError::NotFound(format!("Expense with id {} not found", id)))?
            .into();

        Ok(found_expense)
    }

    async fn find_many(
        &self,
        _filter: Option<FindExpensesDto>,
    ) -> Result<Vec<ExpenseFromDb>, AppError> {
        let found_expenses = self
            .prisma_client
            .expense()
            .find_many(vec![])
            .exec()
            .await?
            .into_iter()
            .map(Into::into)
            .collect();

        Ok(found_expenses)
    }

    async fn create_many(
        &self,
        create_dto: Vec<CreateExpenseDbDto>,
        customer_id: &str,
    ) -> Result<Vec<ExpenseFromDb>, AppError> {
        let prisma_create_dto = create_dto
            .into_iter()
            .map(|dto| {
                expense::create_unchecked(
                    dto.customer_id,
                    dto.amount,
                    dto.date,
                    dto.category.into(),
                    vec![],
                )
            })
            .collect();

        let created_expenses_amount = self
            .prisma_client
            .expense()
            .create_many(prisma_create_dto)
            .exec()
            .await?;

        let created_expenses = self
            .prisma_client
            .expense()
            .find_many(vec![expense::customer_id::equals(customer_id.into())])
            .take(created_expenses_amount)
            .order_by(expense::created_at::order(Direction::Desc))
            .exec()
            .await?
            .into_iter()
            .map(ExpenseFromDb::from)
            .collect();

        Ok(created_expenses)
    }

    async fn update_one(
        &self,
        id: &str,
        update_dto: UpdateExpenseDbDto,
    ) -> Result<ExpenseFromDb, AppError> {
        let updated_expense = self
            .prisma_client
            .expense()
            .update(expense::id::equals(id.into()), update_dto.into())
            .exec()
            .await?
            .into();

        Ok(updated_expense)
    }

    async fn delete_one(&self, id: &str) -> Result<ExpenseFromDb, AppError> {
        let deleted_expense = self
            .prisma_client
            .expense()
            .delete(expense::id::equals(id.into()))
            .exec()
            .await?
            .into();

        Ok(deleted_expense)
    }
}
