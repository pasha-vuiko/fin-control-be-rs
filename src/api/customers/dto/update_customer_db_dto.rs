use chrono::{DateTime, FixedOffset};
use serde::{Deserialize, Serialize};

use crate::api::customers::enums::sex::Sex;
use crate::prisma::customer;

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

impl From<UpdateCustomerDbDto> for Vec<customer::SetParam> {
    fn from(value: UpdateCustomerDbDto) -> Self {
        let mut set_params: Vec<customer::SetParam> = vec![];

        if let Some(auth_0_id) = value.user_id {
            set_params.push(customer::user_id::set(auth_0_id));
        }
        if let Some(first_name) = value.first_name {
            set_params.push(customer::first_name::set(first_name));
        }
        if let Some(last_name) = value.last_name {
            set_params.push(customer::last_name::set(last_name));
        }
        if let Some(email) = value.email {
            set_params.push(customer::email::set(email));
        }
        if let Some(birthdate) = value.birthdate {
            set_params.push(customer::birthdate::set(birthdate));
        }
        if let Some(sex) = value.sex {
            set_params.push(customer::sex::set(sex.into()))
        }
        set_params.push(customer::phone::set(value.phone));

        set_params
    }
}
