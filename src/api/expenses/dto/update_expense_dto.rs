use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::api::expenses::types::expense_category::ExpenseCategory;

#[derive(Debug, Deserialize, Serialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct UpdateExpenseDto {
    #[validate(range(min = 0.0, message = "Should be more than 0"))]
    pub amount: Option<f64>,

    pub date: Option<chrono::DateTime<chrono::FixedOffset>>,

    pub category: Option<ExpenseCategory>,
}
