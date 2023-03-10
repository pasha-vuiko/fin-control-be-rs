use crate::api::customers::dto::create_customer_db_dto::CreateCustomerDbDto;
use crate::api::customers::dto::update_customer_db_dto::UpdateCustomerDbDto;
use crate::api::customers::{
    dto::create_customer_dto::CreateCustomerDto, dto::update_customer_dto::UpdateCustomerDto,
    entities::customer_entity::CustomerEntity, structs::customer_from_db::CustomerFromDb,
    traits::customers_repository::CustomersRepositoryTrait,
};
use crate::shared::errors::app_error::AppError;
use std::sync::Arc;

#[derive(Clone)]
pub struct CustomersService {
    customers_repository: Arc<dyn CustomersRepositoryTrait + Send + Sync>,
}
impl CustomersService {
    pub fn new(customers_repository: Arc<dyn CustomersRepositoryTrait + Send + Sync>) -> Self {
        Self {
            customers_repository,
        }
    }

    fn map_customer_from_db_to_customer_entity(customer_from_db: CustomerFromDb) -> CustomerEntity {
        CustomerEntity {
            id: customer_from_db.id,
            auth_0_id: "mockAuth0Id".to_string(),
            first_name: customer_from_db.first_name,
            last_name: customer_from_db.last_name,
            email: customer_from_db.email,
            birthdate: customer_from_db.birthdate,
            phone: customer_from_db.phone,
            sex: customer_from_db.sex,
        }
    }

    fn map_create_dto_to_create_db_dto(
        create_dto: CreateCustomerDto,
        auth_0_id: String,
        email: String,
    ) -> CreateCustomerDbDto {
        CreateCustomerDbDto {
            auth_0_id,
            email,
            first_name: create_dto.first_name,
            last_name: create_dto.last_name,
            birthdate: create_dto.birthdate,
            phone: create_dto.phone,
            sex: create_dto.sex,
        }
    }

    fn map_update_dto_to_update_db_dto(
        update_dto: UpdateCustomerDto,
        auth_0_id: String,
        email: String,
    ) -> UpdateCustomerDbDto {
        UpdateCustomerDbDto {
            auth_0_id: Some(auth_0_id),
            email: Some(email),
            first_name: update_dto.first_name,
            last_name: update_dto.last_name,
            birthdate: update_dto.birthdate,
            phone: update_dto.phone,
            sex: update_dto.sex,
        }
    }

    pub async fn find_one(&self, id: &str) -> Result<CustomerEntity, AppError> {
        let customer_from_db = self.customers_repository.find_one(id).await?;

        let customer_entity =
            CustomersService::map_customer_from_db_to_customer_entity(customer_from_db);

        Ok(customer_entity)
    }

    pub async fn find_many(&self) -> Result<Vec<CustomerEntity>, AppError> {
        let customers_from_db = self.customers_repository.find_many().await?;

        let customer_entities = customers_from_db
            .into_iter()
            .map(|customer_from_db| {
                CustomersService::map_customer_from_db_to_customer_entity(customer_from_db)
            })
            .collect();

        Ok(customer_entities)
    }

    pub async fn create(&self, create_dto: CreateCustomerDto) -> Result<CustomerEntity, AppError> {
        let create_customer_db_dto = CustomersService::map_create_dto_to_create_db_dto(
            create_dto,
            "mockAuth0Id".to_string(),
            "mock@gmail.com".to_string(),
        );
        let created_customer_from_db = self
            .customers_repository
            .create(create_customer_db_dto)
            .await?;

        let created_customer_entity =
            CustomersService::map_customer_from_db_to_customer_entity(created_customer_from_db);

        Ok(created_customer_entity)
    }

    pub async fn update(
        &self,
        id: &str,
        update_dto: UpdateCustomerDto,
    ) -> Result<CustomerEntity, AppError> {
        let update_db_dto = CustomersService::map_update_dto_to_update_db_dto(
            update_dto,
            "mockAuth0Id".to_string(),
            "mock@gmail.com".to_string(),
        );
        let updated_customer_from_db = self.customers_repository.update(id, update_db_dto).await?;

        let updated_customer_entity =
            CustomersService::map_customer_from_db_to_customer_entity(updated_customer_from_db);

        Ok(updated_customer_entity)
    }

    pub async fn delete(&self, id: &str) -> Result<CustomerEntity, AppError> {
        let deleted_customer_from_db = self.customers_repository.delete(id).await?;

        let deleted_customer_entity =
            CustomersService::map_customer_from_db_to_customer_entity(deleted_customer_from_db);

        Ok(deleted_customer_entity)
    }
}
