use futures_util::future::try_join;
use std::sync::Arc;

use crate::api::customers::customers_service::CustomersService;
use crate::api::expenses::dto::create_expense_db_dto::CreateExpenseDbDto;
use crate::api::expenses::dto::create_expense_dto::CreateExpenseDto;
use crate::api::expenses::dto::find_expenses_dto::FindExpensesDto;
use crate::api::expenses::dto::update_expense_dto::UpdateExpenseDto;
use crate::api::expenses::entities::expense_entity::ExpenseEntity;
use crate::api::expenses::traits::expenses_repository::ExpensesRepositoryTrait;
use crate::shared::errors::app_error::AppError;

#[derive(Clone)]
pub struct ExpensesService {
    pub expenses_repository: Arc<dyn ExpensesRepositoryTrait + Send + Sync>,
    pub customers_service: Arc<CustomersService>,
}

impl ExpensesService {
    pub fn new(
        expenses_repository: Arc<dyn ExpensesRepositoryTrait + Send + Sync>,
        customers_service: Arc<CustomersService>,
    ) -> Self {
        Self {
            expenses_repository,
            customers_service,
        }
    }

    pub async fn find_one_as_admin(&self, id: &str) -> Result<ExpenseEntity, AppError> {
        let expense_entity = self.expenses_repository.find_one(id).await?.into();

        Ok(expense_entity)
    }

    pub async fn find_one_as_customer(
        &self,
        id: &str,
        user_id: &str,
    ) -> Result<ExpenseEntity, AppError> {
        let (customer, expense_from_db) = try_join(
            self.customers_service.find_one_by_user_id(user_id),
            self.expenses_repository.find_one(id),
        )
        .await?;

        if expense_from_db.customer_id != customer.id {
            return Err(AppError::NotFound(format!(
                "Expense with id {} not found",
                id
            )));
        }

        let expense_entity = expense_from_db.into();

        Ok(expense_entity)
    }

    pub async fn find_many(&self) -> Result<Vec<ExpenseEntity>, AppError> {
        let expenses_from_db = self.expenses_repository.find_many(None).await?;

        let expense_entities = expenses_from_db.into_iter().map(Into::into).collect();

        Ok(expense_entities)
    }

    pub async fn find_many_as_customer(
        &self,
        user_id: &str,
    ) -> Result<Vec<ExpenseEntity>, AppError> {
        let customer = self.customers_service.find_one_by_user_id(user_id).await?;

        let find_dto = FindExpensesDto {
            customer_id: Some(customer.id),
        };
        let expenses_from_db = self.expenses_repository.find_many(Some(find_dto)).await?;

        let expense_entities = expenses_from_db.into_iter().map(Into::into).collect();

        Ok(expense_entities)
    }

    pub async fn create_many(
        &self,
        create_dtos: Vec<CreateExpenseDto>,
        user_id: &str,
    ) -> Result<Vec<ExpenseEntity>, AppError> {
        let customer = self.customers_service.find_one_by_user_id(user_id).await?;
        let create_dtos = create_dtos
            .into_iter()
            .map(|create_dto| Self::map_create_dto_to_create_db_dto(create_dto, &customer.id))
            .collect();

        let created_expenses_from_db = self
            .expenses_repository
            .create_many(create_dtos, &customer.id)
            .await?;

        let created_expenses_entity = created_expenses_from_db
            .into_iter()
            .map(Into::into)
            .collect();

        Ok(created_expenses_entity)
    }

    pub async fn update(
        &self,
        id: &str,
        update_dto: UpdateExpenseDto,
        user_id: &str,
    ) -> Result<ExpenseEntity, AppError> {
        // Checking if expense exists and belongs to the customer
        self.find_one_as_customer(id, user_id).await?;

        let updated_expense_entity = self
            .expenses_repository
            .update_one(id, update_dto.into())
            .await?
            .into();

        Ok(updated_expense_entity)
    }

    pub async fn delete(&self, id: &str, user_id: &str) -> Result<ExpenseEntity, AppError> {
        // Checking if expense exists and belongs to the customer
        self.find_one_as_customer(id, user_id).await?;

        let deleted_expense_entity = self.expenses_repository.delete_one(id).await?.into();

        Ok(deleted_expense_entity)
    }

    fn map_create_dto_to_create_db_dto(
        create_dto: CreateExpenseDto,
        customer_id: &str,
    ) -> CreateExpenseDbDto {
        CreateExpenseDbDto {
            customer_id: customer_id.to_string(),
            amount: create_dto.amount,
            date: create_dto.date,
            category: create_dto.category,
        }
    }
}
