use async_trait::async_trait;
use std::sync::Arc;

use crate::api::customers::dto::create_customer_db_dto::CreateCustomerDbDto;
use crate::api::customers::dto::update_customer_db_dto::UpdateCustomerDbDto;
use crate::api::customers::structs::customer_from_db::CustomerFromDb;
use crate::api::customers::traits::customers_repository::CustomersRepositoryTrait;

use crate::shared::errors::app_error::AppError;
use crate::shared::mods::prisma::{customer, PrismaClient};

#[derive(Clone)]
pub struct CustomerRepository {
    prisma_client: Arc<PrismaClient>,
}

impl CustomerRepository {
    pub fn new(prisma_client: Arc<PrismaClient>) -> Self {
        Self { prisma_client }
    }

    fn map_update_dto_to_prisma_update_vec(
        update_dto: UpdateCustomerDbDto,
    ) -> Vec<customer::SetParam> {
        let mut update_values: Vec<customer::SetParam> = vec![];

        if let Some(auth_0_id) = update_dto.user_id {
            update_values.push(customer::user_id::set(auth_0_id));
        }
        if let Some(first_name) = update_dto.first_name {
            update_values.push(customer::first_name::set(first_name));
        }
        if let Some(last_name) = update_dto.last_name {
            update_values.push(customer::last_name::set(last_name));
        }
        if let Some(email) = update_dto.email {
            update_values.push(customer::email::set(email));
        }
        if let Some(birthdate) = update_dto.birthdate {
            update_values.push(customer::birthdate::set(birthdate));
        }
        if let Some(sex) = update_dto.sex {
            update_values.push(customer::sex::set(sex.into()))
        }
        update_values.push(customer::phone::set(update_dto.phone));

        update_values
    }
}

#[async_trait]
impl CustomersRepositoryTrait for CustomerRepository {
    async fn find_one(&self, id: &str) -> Result<CustomerFromDb, AppError> {
        let customer_from_prisma_option = self
            .prisma_client
            .customer()
            .find_unique(customer::id::equals(id.into()))
            .exec()
            .await?;

        match customer_from_prisma_option {
            Some(customer) => Ok(customer.into()),
            None => Err(AppError::NotFound {
                message: format!("Customer with id '{}' was not found", id),
            }),
        }
    }

    async fn find_one_by_user_id(&self, user_id: &str) -> Result<CustomerFromDb, AppError> {
        let customer_from_prisma_option = self
            .prisma_client
            .customer()
            .find_unique(customer::user_id::equals(user_id.into()))
            .exec()
            .await?;

        match customer_from_prisma_option {
            Some(customer) => Ok(customer.into()),
            None => Err(AppError::NotFound {
                message: format!("Customer with user_id '{}' was not found", user_id),
            }),
        }
    }

    async fn find_many(&self) -> Result<Vec<CustomerFromDb>, AppError> {
        let customers_from_prisma = self
            .prisma_client
            .customer()
            .find_many(vec![])
            .exec()
            .await?;

        let mapped_customers = customers_from_prisma
            .into_iter()
            .map(|customer_from_prisma| customer_from_prisma.into())
            .collect();

        Ok(mapped_customers)
    }

    async fn create(&self, create_dto: CreateCustomerDbDto) -> Result<CustomerFromDb, AppError> {
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
    ) -> Result<CustomerFromDb, AppError> {
        let update_values = CustomerRepository::map_update_dto_to_prisma_update_vec(update_dto);

        let updated_customer_from_prisma = self
            .prisma_client
            .customer()
            .update(customer::id::equals(id.into()), update_values)
            .exec()
            .await?;

        Ok(updated_customer_from_prisma.into())
    }

    async fn delete(&self, id: &str) -> Result<CustomerFromDb, AppError> {
        let deleted_customer = self
            .prisma_client
            .customer()
            .delete(customer::id::equals(id.into()))
            .exec()
            .await?;

        Ok(deleted_customer.into())
    }
}
