use async_trait::async_trait;

use crate::api::customers::dto::create_customer_db_dto::CreateCustomerDbDto;
use crate::api::customers::dto::update_customer_db_dto::UpdateCustomerDbDto;
use crate::api::customers::structs::customer_from_db::CustomerFromDb;
use crate::shared::errors::app_error::AppError;

#[async_trait]
pub trait CustomersRepositoryTrait {
    async fn find_one(&self, id: &str) -> Result<CustomerFromDb, AppError>;

    async fn find_one_by_user_id(&self, user_id: &str) -> Result<CustomerFromDb, AppError>;

    async fn find_many(&self) -> Result<Vec<CustomerFromDb>, AppError>;

    async fn create(&self, create_dto: CreateCustomerDbDto) -> Result<CustomerFromDb, AppError>;

    async fn update(
        &self,
        id: &str,
        update_dto: UpdateCustomerDbDto,
    ) -> Result<CustomerFromDb, AppError>;

    async fn delete(&self, id: &str) -> Result<CustomerFromDb, AppError>;
}
