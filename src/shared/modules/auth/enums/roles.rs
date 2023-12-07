use serde::{Deserialize, Deserializer, Serialize};

use crate::shared::errors::http_error::HttpError;

#[derive(Debug, Clone, PartialEq, Serialize)]
pub enum Roles {
    Admin,
    Customer,
}

impl TryFrom<String> for Roles {
    type Error = HttpError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        match value.as_str() {
            "admin" | "ADMIN" | "Admin" => Ok(Roles::Admin),
            "customer" | "CUSTOMER" | "Customer" => Ok(Roles::Customer),
            _ => Err(HttpError::Internal(format!(
                "Role '{value}' is not supported"
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
