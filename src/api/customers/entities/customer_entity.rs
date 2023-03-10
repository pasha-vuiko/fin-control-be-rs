use crate::api::customers::enums::sex::Sex;
use chrono::{DateTime, FixedOffset};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CustomerEntity {
    pub id: String,
    pub auth_0_id: String,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub birthdate: DateTime<FixedOffset>,
    pub phone: Option<String>,
    pub sex: Sex,
}
