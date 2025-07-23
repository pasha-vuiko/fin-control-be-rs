use crate::shared::modules::db::entities::sea_orm_active_enums;
use schemars::JsonSchema;
use sea_orm::{ActiveValue, IntoActiveValue};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, JsonSchema)]
pub enum ExpenseCategory {
    #[serde(rename = "FOOD")]
    Food,
    #[serde(rename = "CLOTHES")]
    Clothes,
    #[serde(rename = "SUBSCRIPTIONS")]
    Subscriptions,
    #[serde(rename = "OTHER")]
    Other,
    #[serde(rename = "UTILITY_PAYMENTS")]
    UtilityPayments,
    #[serde(rename = "ANIMALS")]
    Animals,
    #[serde(rename = "PLACES_TO_EAT")]
    PlacesToEat,
    #[serde(rename = "EDUCATION")]
    Education,
    #[serde(rename = "BOOKS")]
    Books,
    #[serde(rename = "TAXI")]
    Taxi,
    #[serde(rename = "GIFTS")]
    Gifts,
    #[serde(rename = "DONATIONS")]
    Donations,
    #[serde(rename = "MOBILE_SERVICES")]
    MobileServices,
    #[serde(rename = "SPORTS")]
    Sports,
    #[serde(rename = "ENTERTAINMENT")]
    Entertainment,
    #[serde(rename = "BEAUTY_AND_CARE")]
    BeautyAndCare,
    #[serde(rename = "HOUSEHOLD")]
    Household,
    #[serde(rename = "PUBLIC_TRANSPORT")]
    PublicTransport,
    #[serde(rename = "TRAVEL")]
    Travel,
    #[serde(rename = "MEDICINE")]
    Medicine,
}

impl From<sea_orm_active_enums::ExpenseCategory> for ExpenseCategory {
    fn from(value: sea_orm_active_enums::ExpenseCategory) -> Self {
        match value {
            sea_orm_active_enums::ExpenseCategory::Food => Self::Food,
            sea_orm_active_enums::ExpenseCategory::Clothes => Self::Clothes,
            sea_orm_active_enums::ExpenseCategory::Subscriptions => Self::Subscriptions,
            sea_orm_active_enums::ExpenseCategory::Other => Self::Other,
            sea_orm_active_enums::ExpenseCategory::UtilityPayments => Self::UtilityPayments,
            sea_orm_active_enums::ExpenseCategory::Animals => Self::Animals,
            sea_orm_active_enums::ExpenseCategory::PlacesToEat => Self::PlacesToEat,
            sea_orm_active_enums::ExpenseCategory::Education => Self::Education,
            sea_orm_active_enums::ExpenseCategory::Books => Self::Books,
            sea_orm_active_enums::ExpenseCategory::Taxi => Self::Taxi,
            sea_orm_active_enums::ExpenseCategory::Gifts => Self::Gifts,
            sea_orm_active_enums::ExpenseCategory::Donations => Self::Donations,
            sea_orm_active_enums::ExpenseCategory::MobileServices => Self::MobileServices,
            sea_orm_active_enums::ExpenseCategory::Sports => Self::Sports,
            sea_orm_active_enums::ExpenseCategory::Entertainment => Self::Entertainment,
            sea_orm_active_enums::ExpenseCategory::BeautyAndCare => Self::BeautyAndCare,
            sea_orm_active_enums::ExpenseCategory::Household => Self::Household,
            sea_orm_active_enums::ExpenseCategory::PublicTransport => Self::PublicTransport,
            sea_orm_active_enums::ExpenseCategory::Travel => Self::Travel,
            sea_orm_active_enums::ExpenseCategory::Medicine => Self::Medicine,
        }
    }
}

impl From<ExpenseCategory> for sea_orm_active_enums::ExpenseCategory {
    fn from(value: ExpenseCategory) -> Self {
        match value {
            ExpenseCategory::Food => Self::Food,
            ExpenseCategory::Clothes => Self::Clothes,
            ExpenseCategory::Subscriptions => Self::Subscriptions,
            ExpenseCategory::Other => Self::Other,
            ExpenseCategory::UtilityPayments => Self::UtilityPayments,
            ExpenseCategory::Animals => Self::Animals,
            ExpenseCategory::PlacesToEat => Self::PlacesToEat,
            ExpenseCategory::Education => Self::Education,
            ExpenseCategory::Books => Self::Books,
            ExpenseCategory::Taxi => Self::Taxi,
            ExpenseCategory::Gifts => Self::Gifts,
            ExpenseCategory::Donations => Self::Donations,
            ExpenseCategory::MobileServices => Self::MobileServices,
            ExpenseCategory::Sports => Self::Sports,
            ExpenseCategory::Entertainment => Self::Entertainment,
            ExpenseCategory::BeautyAndCare => Self::BeautyAndCare,
            ExpenseCategory::Household => Self::Household,
            ExpenseCategory::PublicTransport => Self::PublicTransport,
            ExpenseCategory::Travel => Self::Travel,
            ExpenseCategory::Medicine => Self::Medicine,
        }
    }
}

impl IntoActiveValue<sea_orm_active_enums::ExpenseCategory> for ExpenseCategory {
    fn into_active_value(self) -> ActiveValue<sea_orm_active_enums::ExpenseCategory> {
        ActiveValue::Set(self.into())
    }
}
