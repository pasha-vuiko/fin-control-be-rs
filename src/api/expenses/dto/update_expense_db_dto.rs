use serde::{Deserialize, Serialize};

use crate::api::expenses::dto::update_expense_dto::UpdateExpenseDto;
use crate::api::expenses::types::expense_category::ExpenseCategory;
use crate::prisma::expense;

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

impl From<UpdateExpenseDbDto> for Vec<expense::SetParam> {
    fn from(value: UpdateExpenseDbDto) -> Self {
        let mut set_params = vec![];

        if let Some(amount) = value.amount {
            set_params.push(expense::amount::set(amount));
        }

        if let Some(date) = value.date {
            set_params.push(expense::date::set(date));
        }

        if let Some(category) = value.category {
            set_params.push(expense::category::set(category.into()));
        }

        set_params
    }
}
