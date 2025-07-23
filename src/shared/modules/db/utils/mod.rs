use sea_orm::{ActiveValue, Value};

pub fn optional_to_active_value<T>(optional: Option<T>) -> ActiveValue<T> where T: Into<Value> {
    match optional {
        Some(value) => ActiveValue::Set(value),
        None => ActiveValue::NotSet
    }
}