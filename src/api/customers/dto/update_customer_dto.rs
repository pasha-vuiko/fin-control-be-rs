use chrono::{DateTime, FixedOffset};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::api::customers::types::sex::Sex;

#[derive(Debug, Deserialize, Serialize, Validate, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct UpdateCustomerDto {
    #[validate(length(min = 1, message = "Can not be empty"))]
    pub first_name: Option<String>,

    #[validate(length(min = 1, message = "Can not be empty"))]
    pub last_name: Option<String>,

    pub birthdate: Option<DateTime<FixedOffset>>,

    pub phone: Option<String>,

    pub sex: Option<Sex>,
}
