use crate::shared::errors::app_error::AppError;
use redis::{aio::ConnectionManager, AsyncCommands, RedisResult, Value};
use std::future::Future;

#[derive(Clone)]
pub struct RedisService {
    redis_connection_manager: ConnectionManager,
}
impl RedisService {
    pub fn new(redis_connection_manager: ConnectionManager) -> Self {
        Self {
            redis_connection_manager,
        }
    }

    pub async fn get<'a, T>(&self, key: &str) -> Result<T, AppError>
    where
        T: serde::Deserialize<'a>,
    {
        let mut redis_connection_manager = self.redis_connection_manager.clone();

        let cached_value = redis_connection_manager.get(key).await?;

        match cached_value {
            // TODO Fix possible memory leak
            Value::Data(value) => match serde_json::from_slice::<T>(value.leak()) {
                Ok(deserialized_value) => Ok(deserialized_value),
                Err(err) => Err(AppError::Internal {
                    message: err.to_string(),
                }),
            },
            Value::Nil => Err(AppError::Internal {
                message: "Value is not found".into(),
            }),
            _ => Err(AppError::Internal {
                message: format!("Some error happen on fetching key '{}'", key),
            }),
        }
    }

    pub async fn set<T>(&self, key: &str, value: &T, expires: usize) -> Result<String, AppError>
    where
        T: serde::Serialize,
    {
        let mut redis_connection_manager = self.redis_connection_manager.clone();

        match serde_json::to_string(value) {
            Ok(serizalized_value) => {
                let redis_result: RedisResult<(String, i32)> = redis::pipe()
                    .atomic()
                    .set(key, &serizalized_value)
                    .expire(key, expires)
                    .query_async(&mut redis_connection_manager)
                    .await;

                match redis_result {
                    Ok(result) => Ok(result.0),
                    Err(err) => Err(AppError::Internal {
                        message: err.to_string(),
                    }),
                }
            }
            Err(err) => Err(AppError::Internal {
                message: format!(
                    "Failed to serialize value for key '{}' to set in Redis, err: '{}'",
                    key,
                    err.to_string()
                ),
            }),
        }
    }

    pub async fn wrap<'a, T, F, Fut>(
        &self,
        func: F,
        cache_key: &str,
        expires: usize,
    ) -> Result<T, AppError>
    where
        T: serde::Serialize + serde::Deserialize<'a>,
        F: FnOnce() -> Fut,
        Fut: Future<Output = Result<T, AppError>>,
    {
        match self.get(cache_key).await {
            Ok(cached_value) => Ok(cached_value),
            Err(_) => {
                let func_value = func().await?;
                let set_result = self.set(cache_key, &func_value, expires).await;

                match set_result {
                    Ok(_) => {
                        tracing::debug!("Cache for endpoint '{}' is set successfully", cache_key)
                    }
                    Err(err) => tracing::warn!(
                        "Cache for endpoint '{}' is failed to set with err: '{}'",
                        cache_key,
                        err
                    ),
                };

                Ok(func_value)
            }
        }
    }
}
