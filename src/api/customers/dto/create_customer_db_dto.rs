use chrono::{DateTime, FixedOffset};
use sea_orm::DeriveIntoActiveModel;
use serde::{Deserialize, Serialize};

use crate::api::customers::types::sex::Sex;

use crate::shared::modules::db::entities::customer::ActiveModel;

#[derive(Serialize, Deserialize, Debug, DeriveIntoActiveModel)]
pub struct CreateCustomerDbDto {
    pub user_id: String,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub phone: Option<String>,
    pub birthdate: DateTime<FixedOffset>,
    pub sex: Sex,
}
