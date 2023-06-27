use async_trait::async_trait;

use crate::api::customers::dto::create_customer_db_dto::CreateCustomerDbDto;
use crate::api::customers::dto::update_customer_db_dto::UpdateCustomerDbDto;
use crate::api::customers::types::customer_from_db::CustomerFromDb;
use crate::shared::errors::http_error::HttpError;

#[async_trait]
pub trait CustomersRepositoryTrait {
    async fn find_one(&self, id: &str) -> Result<CustomerFromDb, HttpError>;

    async fn find_one_by_user_id(&self, user_id: &str) -> Result<CustomerFromDb, HttpError>;

    async fn find_many(&self) -> Result<Vec<CustomerFromDb>, HttpError>;

    async fn create(&self, create_dto: CreateCustomerDbDto) -> Result<CustomerFromDb, HttpError>;

    async fn update(
        &self,
        id: &str,
        update_dto: UpdateCustomerDbDto,
    ) -> Result<CustomerFromDb, HttpError>;

    async fn delete(&self, id: &str) -> Result<CustomerFromDb, HttpError>;
}
