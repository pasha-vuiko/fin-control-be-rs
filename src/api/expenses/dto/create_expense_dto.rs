use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::api::expenses::types::expense_category::ExpenseCategory;

#[derive(Debug, Deserialize, Serialize, Validate, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct CreateExpenseDto {
    #[validate(range(min = 0.0, message = "Should be more than 0"))]
    pub amount: f64,

    pub date: chrono::DateTime<chrono::FixedOffset>,

    pub category: ExpenseCategory,
}
