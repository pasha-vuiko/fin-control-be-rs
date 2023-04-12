use async_trait::async_trait;
use redis::{aio::ConnectionManager, AsyncCommands, RedisResult, Value};
use serde::de::DeserializeOwned;
use serde::Serialize;
use std::future::Future;

use crate::shared::mods::cache::enums::errors::CacheError;
use crate::shared::mods::cache::traits::cache_service::CacheService;

#[derive(Clone)]
pub struct RedisService {
    pub redis_connection_manager: ConnectionManager,
    default_ttl: usize,
}

impl RedisService {
    pub fn new(redis_connection_manager: ConnectionManager, default_ttl: usize) -> Self {
        Self {
            redis_connection_manager,
            default_ttl,
        }
    }
}

#[async_trait]
impl CacheService for RedisService {
    async fn get_str(&self, key: &str) -> Result<String, CacheError> {
        let mut redis_connection_manager = self.redis_connection_manager.clone();
        let cached_value = redis_connection_manager.get(key).await?;

        match cached_value {
            Value::Data(value) => match String::from_utf8(value) {
                Ok(str) => Ok(str),
                Err(err) => Err(CacheError::Unknown(err.to_string())),
            },
            Value::Nil => Err(CacheError::KeyNotFound(
                "Value is not found by the key".into(),
            )),
            _ => Err(CacheError::Unknown(format!(
                "Some error happen on fetching key '{}'",
                key
            ))),
        }
    }

    async fn get<T>(&self, key: &str) -> Result<T, CacheError>
    where
        T: serde::de::DeserializeOwned,
    {
        let mut redis_connection_manager = self.redis_connection_manager.clone();
        let cached_value = redis_connection_manager.get(key).await?;

        match cached_value {
            Value::Data(value) => match serde_json::from_slice::<T>(&value) {
                Ok(deserialized_value) => Ok(deserialized_value),
                Err(err) => Err(CacheError::Unknown(err.to_string())),
            },
            Value::Nil => Err(CacheError::KeyNotFound("Value is not found".into())),
            _ => Err(CacheError::Unknown(format!(
                "Some error happen on fetching key '{}'",
                key
            ))),
        }
    }

    async fn set<T>(&self, key: &str, value: &T) -> Result<String, CacheError>
    where
        T: Serialize + Send + Sync,
    {
        let mut redis_connection_manager = self.redis_connection_manager.clone();

        match serde_json::to_string(value) {
            Ok(serialized_value) => {
                let redis_result: RedisResult<(String, i32)> = redis::pipe()
                    .atomic()
                    .set(key, &serialized_value)
                    .expire(key, self.default_ttl)
                    .query_async(&mut redis_connection_manager)
                    .await;

                match redis_result {
                    Ok(result) => Ok(result.0),
                    Err(err) => Err(CacheError::Unknown(err.to_string())),
                }
            }
            Err(err) => Err(CacheError::Unknown(format!(
                "Failed to serialize value for key '{}' to set in Redis, err: '{}'",
                key, err
            ))),
        }
    }

    async fn set_with_ttl<T>(&self, key: &str, value: &T, ttl: usize) -> Result<String, CacheError>
    where
        T: Serialize + Send + Sync,
    {
        let mut redis_connection_manager = self.redis_connection_manager.clone();

        match serde_json::to_string(value) {
            Ok(serialized_value) => {
                let redis_result: RedisResult<(String, i32)> = redis::pipe()
                    .atomic()
                    .set(key, &serialized_value)
                    .expire(key, ttl)
                    .query_async(&mut redis_connection_manager)
                    .await;

                match redis_result {
                    Ok(result) => Ok(result.0),
                    Err(err) => Err(CacheError::Unknown(err.to_string())),
                }
            }
            Err(err) => Err(CacheError::Unknown(format!(
                "Failed to serialize value for key '{}' to set in Redis, err: '{}'",
                key, err
            ))),
        }
    }

    async fn set_str(&self, key: &str, value: &str) -> Result<String, CacheError> {
        let mut redis_connection_manager = self.redis_connection_manager.clone();

        let redis_result: RedisResult<(String, i32)> = redis::pipe()
            .atomic()
            .set(key, value)
            .expire(key, self.default_ttl)
            .query_async(&mut redis_connection_manager)
            .await;

        match redis_result {
            Ok(result) => Ok(result.0),
            Err(err) => Err(CacheError::Unknown(err.to_string())),
        }
    }

    async fn set_str_with_ttl(
        &self,
        key: &str,
        value: &str,
        ttl: usize,
    ) -> Result<String, CacheError> {
        let mut redis_connection_manager = self.redis_connection_manager.clone();

        let redis_result: RedisResult<(String, i32)> = redis::pipe()
            .atomic()
            .set(key, value)
            .expire(key, ttl)
            .query_async(&mut redis_connection_manager)
            .await;

        match redis_result {
            Ok(result) => Ok(result.0),
            Err(err) => Err(CacheError::Unknown(err.to_string())),
        }
    }

    async fn wrap_fn<T, F, Fut>(&self, func: F, cache_key: &str) -> Result<T, CacheError>
    where
        T: Serialize + DeserializeOwned + Send + Sync,
        F: FnOnce() -> Fut + Send + Sync,
        Fut: Future<Output = Result<T, CacheError>> + Send + Sync,
    {
        match self.get(cache_key).await {
            Ok(cached_value) => Ok(cached_value),
            Err(_) => {
                let func_value = func().await?;
                let set_result = self.set(cache_key, &func_value).await;

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
