use chrono::{DateTime, FixedOffset};
use serde::{Deserialize, Serialize};

use crate::api::customers::enums::sex::Sex;
use crate::shared::mods::prisma::customer;

#[derive(Serialize, Deserialize, Debug)]
pub struct CustomerFromDb {
    pub id: String,
    pub user_id: String,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub phone: Option<String>,
    pub birthdate: DateTime<FixedOffset>,
    pub sex: Sex,
}

impl From<customer::Data> for CustomerFromDb {
    fn from(value: customer::Data) -> Self {
        Self {
            id: value.id,
            user_id: value.user_id,
            first_name: value.first_name,
            last_name: value.last_name,
            email: value.email,
            phone: value.phone,
            birthdate: value.birthdate,
            sex: value.sex.into(),
        }
    }
}
