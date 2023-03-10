use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub enum Sex {
    MALE,
    FEMALE,
}
