use crate::api::customers::enums::sex::Sex;
use chrono::{DateTime, FixedOffset};
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Debug, Deserialize, Serialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct CreateCustomerDto {
    #[validate(length(min = 1, message = "Can not be empty"))]
    pub first_name: String,

    #[validate(length(min = 1, message = "Can not be empty"))]
    pub last_name: String,

    pub birthdate: DateTime<FixedOffset>,

    #[validate(phone)]
    pub phone: Option<String>,

    pub sex: Sex,
}
