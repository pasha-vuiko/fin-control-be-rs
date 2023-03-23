use crate::shared::mods::prisma;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub enum Sex {
    Male,
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

impl Into<prisma::Sex> for Sex {
    fn into(self) -> prisma::Sex {
        match self {
            Self::Male => prisma::Sex::Male,
            Self::Female => prisma::Sex::Female,
        }
    }
}
