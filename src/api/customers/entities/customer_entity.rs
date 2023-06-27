use crate::api::customers::types::{customer_from_db::CustomerFromDb, sex::Sex};
use chrono::{DateTime, FixedOffset};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CustomerEntity {
    pub id: String,
    pub user_id: String,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub birthdate: DateTime<FixedOffset>,
    pub phone: Option<String>,
    pub sex: Sex,
}

impl From<CustomerFromDb> for CustomerEntity {
    fn from(value: CustomerFromDb) -> Self {
        Self {
            id: value.id,
            user_id: value.user_id,
            first_name: value.first_name,
            last_name: value.last_name,
            email: value.email,
            birthdate: value.birthdate,
            phone: value.phone,
            sex: value.sex,
        }
    }
}
