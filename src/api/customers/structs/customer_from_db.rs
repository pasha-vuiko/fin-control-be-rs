use chrono::{DateTime, FixedOffset};
use serde::{Deserialize, Serialize};

use crate::api::customers::enums::sex::Sex;

#[derive(Serialize, Deserialize, Debug)]
pub struct CustomerFromDb {
    pub id: String,
    pub auth_0_id: String,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub phone: Option<String>,
    pub birthdate: DateTime<FixedOffset>,
    pub sex: Sex,
}
