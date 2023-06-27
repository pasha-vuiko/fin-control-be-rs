use serde::{Deserialize, Serialize};

use crate::api::expenses::types::expense_category::ExpenseCategory;
use crate::api::expenses::types::expense_from_db::ExpenseFromDb;

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ExpenseEntity {
    pub id: String,
    pub customer_id: String,
    pub amount: f64,
    pub date: chrono::DateTime<chrono::FixedOffset>,
    pub category: ExpenseCategory,
    pub created_at: chrono::DateTime<chrono::FixedOffset>,
    pub updated_at: chrono::DateTime<chrono::FixedOffset>,
}

impl From<ExpenseFromDb> for ExpenseEntity {
    fn from(value: ExpenseFromDb) -> Self {
        Self {
            id: value.id,
            customer_id: value.customer_id,
            amount: value.amount,
            date: value.date,
            category: value.category,
            created_at: value.created_at,
            updated_at: value.updated_at,
        }
    }
}
