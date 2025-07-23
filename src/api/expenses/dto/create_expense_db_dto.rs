use sea_orm::{DeriveIntoActiveModel};
use sea_orm::prelude::Decimal;
use serde::{Deserialize, Serialize};
use crate::api::expenses::types::expense_category::ExpenseCategory;

use crate::shared::modules::db::entities::expense::ActiveModel;

#[derive(Serialize, Deserialize, Debug, DeriveIntoActiveModel)]
pub struct CreateExpenseDbDto {
    pub customer_id: String,
    pub amount: Decimal,
    pub date: chrono::DateTime<chrono::FixedOffset>,
    pub category: ExpenseCategory,
}
