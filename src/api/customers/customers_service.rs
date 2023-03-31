use crate::api::customers::dto::create_customer_db_dto::CreateCustomerDbDto;
use crate::api::customers::dto::update_customer_db_dto::UpdateCustomerDbDto;
use crate::api::customers::{
    dto::create_customer_dto::CreateCustomerDto, dto::update_customer_dto::UpdateCustomerDto,
    entities::customer_entity::CustomerEntity,
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

    pub async fn find_one_by_id(&self, id: &str) -> Result<CustomerEntity, AppError> {
        let customer_from_db = self.customers_repository.find_one(id).await?;
        let customer_entity = customer_from_db.into();

        Ok(customer_entity)
    }

    pub async fn find_one_by_user_id(&self, user_id: &str) -> Result<CustomerEntity, AppError> {
        let customer_from_db = self
            .customers_repository
            .find_one_by_user_id(user_id)
            .await?;
        let customer_entity = customer_from_db.into();

        Ok(customer_entity)
    }

    pub async fn find_many(&self) -> Result<Vec<CustomerEntity>, AppError> {
        let customers_from_db = self.customers_repository.find_many().await?;

        let customer_entities = customers_from_db
            .into_iter()
            .map(|customer_from_db| customer_from_db.into())
            .collect();

        Ok(customer_entities)
    }

    pub async fn create(
        &self,
        create_dto: CreateCustomerDto,
        user_id: &str,
        email: &str,
    ) -> Result<CustomerEntity, AppError> {
        let create_customer_db_dto =
            CustomersService::map_create_dto_to_create_db_dto(create_dto, user_id, email);
        let created_customer_from_db = self
            .customers_repository
            .create(create_customer_db_dto)
            .await?;

        let created_customer_entity = created_customer_from_db.into();

        Ok(created_customer_entity)
    }

    pub async fn update_as_customer(
        &self,
        id: &str,
        update_dto: UpdateCustomerDto,
        user_id: &str,
    ) -> Result<CustomerEntity, AppError> {
        let found_customer = self.customers_repository.find_one(id).await?;

        if found_customer.user_id != user_id {
            return Err(AppError::NotFound {
                message: "'The customer was not found'".into(),
            });
        }

        let update_db_dto = CustomersService::map_update_dto_to_update_db_dto(
            update_dto,
            "mockAuth0Id".into(),
            "mock@gmail.com".into(),
        );
        let updated_customer_from_db = self.customers_repository.update(id, update_db_dto).await?;

        let updated_customer_entity = updated_customer_from_db.into();

        Ok(updated_customer_entity)
    }

    pub async fn update_as_admin(
        &self,
        id: &str,
        update_dto: UpdateCustomerDto,
    ) -> Result<CustomerEntity, AppError> {
        let update_db_dto = CustomersService::map_update_dto_to_update_db_dto(
            update_dto,
            "mockAuth0Id".into(),
            "mock@gmail.com".into(),
        );
        let updated_customer_from_db = self.customers_repository.update(id, update_db_dto).await?;

        let updated_customer_entity = updated_customer_from_db.into();

        Ok(updated_customer_entity)
    }

    pub async fn delete_as_admin(&self, id: &str) -> Result<CustomerEntity, AppError> {
        let deleted_customer_from_db = self.customers_repository.delete(id).await?;
        let deleted_customer_entity = deleted_customer_from_db.into();

        Ok(deleted_customer_entity)
    }

    pub async fn delete_as_customer(
        &self,
        id: &str,
        user_id: &str,
    ) -> Result<CustomerEntity, AppError> {
        let found_customer = self.customers_repository.find_one(id).await?;

        if found_customer.user_id != user_id {
            return Err(AppError::NotFound {
                message: "'The customer was not found'".into(),
            });
        }

        let deleted_customer_from_db = self.customers_repository.delete(id).await?;
        let deleted_customer_entity = deleted_customer_from_db.into();

        Ok(deleted_customer_entity)
    }

    fn map_create_dto_to_create_db_dto(
        create_dto: CreateCustomerDto,
        user_id: &str,
        email: &str,
    ) -> CreateCustomerDbDto {
        CreateCustomerDbDto {
            user_id: user_id.into(),
            email: email.into(),
            first_name: create_dto.first_name,
            last_name: create_dto.last_name,
            birthdate: create_dto.birthdate,
            phone: create_dto.phone,
            sex: create_dto.sex,
        }
    }

    fn map_update_dto_to_update_db_dto(
        update_dto: UpdateCustomerDto,
        user_id: String,
        email: String,
    ) -> UpdateCustomerDbDto {
        UpdateCustomerDbDto {
            user_id: Some(user_id),
            email: Some(email),
            first_name: update_dto.first_name,
            last_name: update_dto.last_name,
            birthdate: update_dto.birthdate,
            phone: update_dto.phone,
            sex: update_dto.sex,
        }
    }
}
