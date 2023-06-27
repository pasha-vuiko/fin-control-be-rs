use serde::{Deserialize, Serialize};

use crate::prisma;

#[derive(Serialize, Deserialize, Debug)]
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

impl From<prisma::ExpenseCategory> for ExpenseCategory {
    fn from(value: prisma::ExpenseCategory) -> Self {
        match value {
            prisma::ExpenseCategory::Food => Self::Food,
            prisma::ExpenseCategory::Clothes => Self::Clothes,
            prisma::ExpenseCategory::Subscriptions => Self::Subscriptions,
            prisma::ExpenseCategory::Other => Self::Other,
            prisma::ExpenseCategory::UtilityPayments => Self::UtilityPayments,
            prisma::ExpenseCategory::Animals => Self::Animals,
            prisma::ExpenseCategory::PlacesToEat => Self::PlacesToEat,
            prisma::ExpenseCategory::Education => Self::Education,
            prisma::ExpenseCategory::Books => Self::Books,
            prisma::ExpenseCategory::Taxi => Self::Taxi,
            prisma::ExpenseCategory::Gifts => Self::Gifts,
            prisma::ExpenseCategory::Donations => Self::Donations,
            prisma::ExpenseCategory::MobileServices => Self::MobileServices,
            prisma::ExpenseCategory::Sports => Self::Sports,
            prisma::ExpenseCategory::Enterainment => Self::Entertainment,
            prisma::ExpenseCategory::BeautyAndCare => Self::BeautyAndCare,
            prisma::ExpenseCategory::Household => Self::Household,
            prisma::ExpenseCategory::PublicTransport => Self::PublicTransport,
            prisma::ExpenseCategory::Travel => Self::Travel,
            prisma::ExpenseCategory::Medicine => Self::Medicine,
        }
    }
}

impl From<ExpenseCategory> for prisma::ExpenseCategory {
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
            ExpenseCategory::Entertainment => Self::Enterainment,
            ExpenseCategory::BeautyAndCare => Self::BeautyAndCare,
            ExpenseCategory::Household => Self::Household,
            ExpenseCategory::PublicTransport => Self::PublicTransport,
            ExpenseCategory::Travel => Self::Travel,
            ExpenseCategory::Medicine => Self::Medicine,
        }
    }
}
