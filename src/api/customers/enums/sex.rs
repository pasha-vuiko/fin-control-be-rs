use crate::shared::mods::prisma;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub enum Sex {
    #[serde(rename = "MALE")]
    Male,
    #[serde(rename = "FEMALE")]
    Female,
}

impl From<prisma::Sex> for Sex {
    fn from(value: prisma::Sex) -> Self {
        match value {
            prisma::Sex::Male => Sex::Male,
            prisma::Sex::Female => Sex::Female,
        }
    }
}

impl From<Sex> for prisma::Sex {
    fn from(value: Sex) -> Self {
        match value {
            Sex::Male => prisma::Sex::Male,
            Sex::Female => prisma::Sex::Female,
        }
    }
}
