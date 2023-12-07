use async_trait::async_trait;
use prisma_client::{customer, PrismaClient};
use std::sync::Arc;

use crate::api::customers::dto::create_customer_db_dto::CreateCustomerDbDto;
use crate::api::customers::dto::update_customer_db_dto::UpdateCustomerDbDto;
use crate::api::customers::traits::customers_repository::CustomersRepositoryTrait;
use crate::api::customers::types::customer_from_db::CustomerFromDb;

use crate::shared::errors::http_error::HttpError;

#[derive(Clone)]
pub struct CustomerRepository {
    prisma_client: Arc<PrismaClient>,
}

impl CustomerRepository {
    pub fn new(prisma_client: Arc<PrismaClient>) -> Self {
        Self { prisma_client }
    }
}

#[async_trait]
impl CustomersRepositoryTrait for CustomerRepository {
    async fn find_one(&self, id: &str) -> Result<CustomerFromDb, HttpError> {
        let customer_from_prisma = self
            .prisma_client
            .customer()
            .find_unique(customer::id::equals(id.into()))
            .exec()
            .await?
            .ok_or_else(|| HttpError::NotFound(format!("Customer with id '{id}' was not found")))?;

        Ok(customer_from_prisma.into())
    }

    async fn find_one_by_user_id(&self, user_id: &str) -> Result<CustomerFromDb, HttpError> {
        let customer_from_prisma = self
            .prisma_client
            .customer()
            .find_unique(customer::user_id::equals(user_id.into()))
            .exec()
            .await?
            .ok_or_else(|| {
                HttpError::NotFound(format!("Customer with user_id '{user_id}' was not found"))
            })?;

        Ok(customer_from_prisma.into())
    }

    async fn find_many(&self) -> Result<Vec<CustomerFromDb>, HttpError> {
        let customers_from_prisma = self
            .prisma_client
            .customer()
            .find_many(vec![])
            .exec()
            .await?;

        let mapped_customers = customers_from_prisma.into_iter().map(Into::into).collect();

        Ok(mapped_customers)
    }

    async fn create(&self, create_dto: CreateCustomerDbDto) -> Result<CustomerFromDb, HttpError> {
        let created_customer_from_prisma = self
            .prisma_client
            .customer()
            .create(
                create_dto.user_id,
                create_dto.first_name,
                create_dto.last_name,
                create_dto.email,
                create_dto.birthdate,
                create_dto.sex.into(),
                vec![],
            )
            .exec()
            .await?;

        Ok(created_customer_from_prisma.into())
    }

    async fn update(
        &self,
        id: &str,
        update_dto: UpdateCustomerDbDto,
    ) -> Result<CustomerFromDb, HttpError> {
        let updated_customer_from_prisma = self
            .prisma_client
            .customer()
            .update(customer::id::equals(id.into()), update_dto.into())
            .exec()
            .await?;

        Ok(updated_customer_from_prisma.into())
    }

    async fn delete(&self, id: &str) -> Result<CustomerFromDb, HttpError> {
        let deleted_customer = self
            .prisma_client
            .customer()
            .delete(customer::id::equals(id.into()))
            .exec()
            .await?;

        Ok(deleted_customer.into())
    }
}
