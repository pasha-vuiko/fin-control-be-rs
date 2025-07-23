use crate::api::customers::dto::create_customer_db_dto::CreateCustomerDbDto;
use crate::api::customers::dto::update_customer_db_dto::UpdateCustomerDbDto;
use crate::api::customers::traits::customers_repository::CustomersRepositoryTrait;
use crate::api::customers::types::customer_from_db::CustomerFromDb;
use async_trait::async_trait;
use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, IntoActiveModel, QueryFilter};
use std::sync::Arc;

use crate::shared::errors::http_error::HttpError;
use crate::shared::modules::db::entities::customer;
use crate::shared::modules::db::entities::customer::ActiveModel as CustomerActiveModel;
use crate::shared::modules::db::entities::prelude::Customer;

#[derive(Clone)]
pub struct CustomerRepository {
    sea_orm_client: Arc<DatabaseConnection>,
}

impl CustomerRepository {
    pub fn new(sea_orm_client: Arc<DatabaseConnection>) -> Self {
        Self { sea_orm_client }
    }
}

#[async_trait]
impl CustomersRepositoryTrait for CustomerRepository {
    async fn find_one(&self, id: &str) -> Result<CustomerFromDb, HttpError> {
        let customer_from_db = Customer::find_by_id(id)
            .one(self.sea_orm_client.as_ref())
            .await?
            .ok_or_else(|| HttpError::NotFound(format!("Customer with id '{id}' was not found")))?;

        Ok(customer_from_db.into())
    }

    async fn find_one_by_user_id(&self, user_id: &str) -> Result<CustomerFromDb, HttpError> {
        let customer_from_db = Customer::find()
            .filter(customer::Column::UserId.eq(user_id))
            .one(self.sea_orm_client.as_ref())
            .await?
            .ok_or_else(|| {
                HttpError::NotFound(format!("Customer with user_id '{user_id}' was not found"))
            })?;

        Ok(customer_from_db.into())
    }

    async fn find_many(&self) -> Result<Vec<CustomerFromDb>, HttpError> {
        let customers_from_db = Customer::find().all(&*self.sea_orm_client).await?;

        let mapped_customers = customers_from_db.into_iter().map(Into::into).collect();

        Ok(mapped_customers)
    }

    async fn create(&self, create_dto: CreateCustomerDbDto) -> Result<CustomerFromDb, HttpError> {
        let created_customer_from_db = Customer::insert(create_dto.into_active_model())
            .exec_with_returning(self.sea_orm_client.as_ref())
            .await?;

        Ok(created_customer_from_db.into())
    }

    async fn update(
        &self,
        id: &str,
        update_dto: UpdateCustomerDbDto,
    ) -> Result<CustomerFromDb, HttpError> {
        let updated_customer_from_db = Customer::update(CustomerActiveModel::from(update_dto))
            .filter(customer::Column::Id.eq(id))
            .exec(self.sea_orm_client.as_ref())
            .await?;

        Ok(updated_customer_from_db.into())
    }

    async fn delete(&self, id: &str) -> Result<CustomerFromDb, HttpError> {
        let deleted_customer = Customer::delete_by_id(id)
            .exec_with_returning(self.sea_orm_client.as_ref())
            .await?
            .first()
            .cloned()
            .ok_or(HttpError::NotFound(format!(
                "Customer with id '{id}' was not found"
            )))?;

        Ok(deleted_customer.clone().into())
    }
}
