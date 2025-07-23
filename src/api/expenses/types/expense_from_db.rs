use serde::{Deserialize, Serialize};

use crate::api::expenses::types::expense_category::ExpenseCategory;
use crate::shared::modules::db::entities::expense;

#[derive(Serialize, Deserialize, Debug)]
pub struct ExpenseFromDb {
    pub id: String,
    pub customer_id: String,
    pub amount: f64,
    pub date: chrono::DateTime<chrono::FixedOffset>,
    pub category: ExpenseCategory,
    pub created_at: chrono::DateTime<chrono::FixedOffset>,
    pub updated_at: chrono::DateTime<chrono::FixedOffset>,
}

impl From<expense::Model> for ExpenseFromDb {
    fn from(value: expense::Model) -> Self {
        Self {
            id: value.id,
            customer_id: value.customer_id,
            amount: value.amount.try_into().unwrap_or_default(),
            date: value.date,
            category: value.category.into(),
            created_at: value.created_at,
            updated_at: value.updated_at,
        }
    }
}
