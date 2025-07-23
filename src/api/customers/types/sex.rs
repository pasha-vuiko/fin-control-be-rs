use crate::shared::modules::db::entities::sea_orm_active_enums;
use schemars::JsonSchema;
use sea_orm::{ActiveValue, IntoActiveValue};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, JsonSchema)]
pub enum Sex {
    #[serde(rename = "MALE")]
    Male,
    #[serde(rename = "FEMALE")]
    Female,
}

impl From<sea_orm_active_enums::Sex> for Sex {
    fn from(value: sea_orm_active_enums::Sex) -> Self {
        match value {
            sea_orm_active_enums::Sex::Male => Sex::Male,
            sea_orm_active_enums::Sex::Female => Sex::Female,
        }
    }
}

impl From<Sex> for sea_orm_active_enums::Sex {
    fn from(value: Sex) -> Self {
        match value {
            Sex::Male => sea_orm_active_enums::Sex::Male,
            Sex::Female => sea_orm_active_enums::Sex::Female,
        }
    }
}

impl IntoActiveValue<sea_orm_active_enums::Sex> for Sex {
    fn into_active_value(self) -> ActiveValue<sea_orm_active_enums::Sex> {
        ActiveValue::Set(self.into())
    }
}
