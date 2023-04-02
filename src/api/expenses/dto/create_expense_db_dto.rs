use serde::{Deserialize, Serialize};

use crate::api::expenses::enums::expense_category::ExpenseCategory;

#[derive(Serialize, Deserialize, Debug)]
pub struct CreateExpenseDbDto {
    pub customer_id: String,
    pub amount: f64,
    pub date: chrono::DateTime<chrono::FixedOffset>,
    pub category: ExpenseCategory,
}
