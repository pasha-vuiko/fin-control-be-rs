use crate::api::expenses::enums::expense_category::ExpenseCategory;
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Debug, Deserialize, Serialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct CreateExpenseDto {
    #[validate(range(min = 0.0, message = "Should be more than 0"))]
    pub amount: f64,

    pub date: chrono::DateTime<chrono::FixedOffset>,

    pub category: ExpenseCategory,
}
