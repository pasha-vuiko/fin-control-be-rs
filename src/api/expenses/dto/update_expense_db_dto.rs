use sea_orm::ActiveValue;
use sea_orm::prelude::Decimal;
use serde::{Deserialize, Serialize};

use crate::api::expenses::dto::update_expense_dto::UpdateExpenseDto;
use crate::api::expenses::types::expense_category::ExpenseCategory;

use crate::shared::modules::db::entities::expense::ActiveModel as ExpenseActiveModel;
use crate::shared::modules::db::entities::sea_orm_active_enums;
use crate::shared::modules::db::utils::optional_to_active_value;

#[derive(Serialize, Deserialize, Debug)]
pub struct UpdateExpenseDbDto {
    pub amount: Option<f64>,
    pub date: Option<chrono::DateTime<chrono::FixedOffset>>,
    pub category: Option<ExpenseCategory>,
}

impl From<UpdateExpenseDto> for UpdateExpenseDbDto {
    fn from(dto: UpdateExpenseDto) -> Self {
        Self {
            amount: dto.amount,
            date: dto.date,
            category: dto.category,
        }
    }
}

impl From<UpdateExpenseDbDto> for ExpenseActiveModel {
    fn from(value: UpdateExpenseDbDto) -> Self {
        Self {
            id: ActiveValue::NotSet,
            customer_id: ActiveValue::NotSet,
            category: optional_to_active_value(
                value
                    .category
                    .map(sea_orm_active_enums::ExpenseCategory::from),
            ),
            amount: optional_to_active_value(
                value
                    .amount
                    .map(|amount| Decimal::from_f64_retain(amount).unwrap_or_default()),
            ),
            date: optional_to_active_value(value.date.into()),
            created_at: ActiveValue::NotSet,
            updated_at: ActiveValue::NotSet,
        }
    }
}
