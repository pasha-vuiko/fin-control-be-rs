use chrono::{DateTime, FixedOffset};
use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::api::customers::enums::sex::Sex;

#[derive(Debug, Deserialize, Serialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct UpdateCustomerDto {
    #[validate(length(min = 1, message = "Can not be empty"))]
    pub first_name: Option<String>,

    #[validate(length(min = 1, message = "Can not be empty"))]
    pub last_name: Option<String>,

    pub birthdate: Option<DateTime<FixedOffset>>,

    #[validate(phone)]
    pub phone: Option<String>,

    pub sex: Option<Sex>,
}
