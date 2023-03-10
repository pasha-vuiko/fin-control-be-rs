use crate::api::customers::enums::sex::Sex;
use chrono::{DateTime, FixedOffset};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct UpdateCustomerDbDto {
    pub auth_0_id: Option<String>,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub email: Option<String>,
    pub phone: Option<String>,
    pub birthdate: Option<DateTime<FixedOffset>>,
    pub sex: Option<Sex>,
}
