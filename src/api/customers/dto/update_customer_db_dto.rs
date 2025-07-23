use crate::api::customers::types::sex::Sex;
use chrono::{DateTime, FixedOffset, Utc};
use sea_orm::ActiveValue;
use serde::{Deserialize, Serialize};

use crate::shared::modules::db::entities::customer::ActiveModel as CustomerActiveModel;
use crate::shared::modules::db::entities::sea_orm_active_enums;
use crate::shared::modules::db::utils::optional_to_active_value;

#[derive(Serialize, Deserialize, Debug)]
pub struct UpdateCustomerDbDto {
    pub user_id: Option<String>,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub email: Option<String>,
    pub phone: Option<String>,
    pub birthdate: Option<DateTime<FixedOffset>>,
    pub sex: Option<Sex>,
}

impl From<UpdateCustomerDbDto> for CustomerActiveModel {
    fn from(value: UpdateCustomerDbDto) -> Self {
        let mapped_sex: Option<sea_orm_active_enums::Sex> = value.sex.map(sea_orm_active_enums::Sex::from);

        Self {
            id: ActiveValue::NotSet,
            user_id: optional_to_active_value(value.user_id),
            email: optional_to_active_value(value.email),
            phone: ActiveValue::Set(value.phone),
            first_name: optional_to_active_value(value.first_name),
            last_name: optional_to_active_value(value.last_name),
            sex: optional_to_active_value(mapped_sex),
            birthdate: optional_to_active_value(value.birthdate.into()),
            updated_at: ActiveValue::Set(Utc::now().into()),
            created_at: ActiveValue::NotSet,
        }
    }
}
