use serde::{Deserialize, Deserializer, Serialize};

use crate::shared::errors::app_error::AppError;

#[derive(Debug, Clone, PartialEq, Serialize)]
pub enum Roles {
    Admin,
    Customer,
}

impl TryFrom<String> for Roles {
    type Error = AppError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        match value.as_str() {
            "admin" | "ADMIN" | "Admin" => Ok(Roles::Admin),
            "customer" | "CUSTOMER" | "Customer" => Ok(Roles::Customer),
            _ => Err(AppError::Internal(format!(
                "Role '{}' is not supported",
                value
            ))),
        }
    }
}

impl<'de> Deserialize<'de> for Roles {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Roles::try_from(s).map_err(serde::de::Error::custom)
    }
}
