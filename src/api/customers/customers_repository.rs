use async_trait::async_trait;
use std::sync::Arc;

use crate::api::customers;
use crate::api::customers::dto::create_customer_db_dto::CreateCustomerDbDto;
use crate::api::customers::dto::update_customer_db_dto::UpdateCustomerDbDto;
use crate::api::customers::structs::customer_from_db::CustomerFromDb;
use crate::api::customers::traits::customers_repository::CustomersRepositoryTrait;

use crate::shared::errors::app_error::AppError;
use crate::shared::mods::prisma;
use crate::shared::mods::prisma::{customer, PrismaClient};

#[derive(Clone)]
pub struct CustomerRepository {
    prisma_client: Arc<PrismaClient>,
}

impl CustomerRepository {
    pub fn new(prisma_client: Arc<PrismaClient>) -> Self {
        Self { prisma_client }
    }

    fn map_customer_from_db(customer_from_prisma: customer::Data) -> CustomerFromDb {
        CustomerFromDb {
            id: customer_from_prisma.id,
            auth_0_id: customer_from_prisma.auth_0_id,
            first_name: customer_from_prisma.first_name,
            last_name: customer_from_prisma.last_name,
            email: customer_from_prisma.email,
            phone: customer_from_prisma.phone,
            birthdate: customer_from_prisma.birthdate,
            sex: CustomerRepository::map_prisma_sex_to_customer_sex(customer_from_prisma.sex),
        }
    }

    fn map_prisma_sex_to_customer_sex(sex_from_prisma: prisma::Sex) -> customers::enums::sex::Sex {
        match sex_from_prisma {
            prisma::Sex::Male => customers::enums::sex::Sex::MALE,
            prisma::Sex::Female => customers::enums::sex::Sex::FEMALE,
        }
    }

    fn map_customer_sex_to_prisma_sex(customer_sex: customers::enums::sex::Sex) -> prisma::Sex {
        match customer_sex {
            customers::enums::sex::Sex::MALE => prisma::Sex::Male,
            customers::enums::sex::Sex::FEMALE => prisma::Sex::Female,
        }
    }

    fn map_update_dto_to_prisma_update_vec(
        update_dto: UpdateCustomerDbDto,
    ) -> Vec<customer::SetParam> {
        let mut update_values: Vec<customer::SetParam> = vec![];

        if let Some(auth_0_id) = update_dto.auth_0_id {
            update_values.push(customer::auth_0_id::set(auth_0_id));
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
            let prisma_sex = CustomerRepository::map_customer_sex_to_prisma_sex(sex);

            update_values.push(customer::sex::set(prisma_sex))
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
            .find_unique(customer::id::equals(id.to_string()))
            .exec()
            .await?;

        match customer_from_prisma_option {
            Some(customer) => {
                let customer = CustomerRepository::map_customer_from_db(customer);

                Ok(customer)
            }
            None => Err(AppError::NotFound {
                message: format!("Customer with id '{}' was not found", id),
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
            .map(|customer_from_prisma| {
                CustomerRepository::map_customer_from_db(customer_from_prisma)
            })
            .collect();

        Ok(mapped_customers)
    }

    async fn create(&self, create_dto: CreateCustomerDbDto) -> Result<CustomerFromDb, AppError> {
        let created_customer_from_prisma = self
            .prisma_client
            .customer()
            .create(
                create_dto.auth_0_id,
                create_dto.first_name,
                create_dto.last_name,
                create_dto.email,
                create_dto.birthdate,
                CustomerRepository::map_customer_sex_to_prisma_sex(create_dto.sex),
                vec![],
            )
            .exec()
            .await?;

        let mapped_created_customer =
            CustomerRepository::map_customer_from_db(created_customer_from_prisma);

        Ok(mapped_created_customer)
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
            .update(prisma::customer::id::equals(id.to_string()), update_values)
            .exec()
            .await?;

        let mapped_updated_customer =
            CustomerRepository::map_customer_from_db(updated_customer_from_prisma);

        Ok(mapped_updated_customer)
    }

    async fn delete(&self, id: &str) -> Result<CustomerFromDb, AppError> {
        let deleted_customer = self
            .prisma_client
            .customer()
            .delete(customer::id::equals(id.to_string()))
            .exec()
            .await?;

        let mapped_customer = CustomerRepository::map_customer_from_db(deleted_customer);

        Ok(mapped_customer)
    }
}
