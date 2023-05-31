use async_trait::async_trait;
use redis::{aio::ConnectionManager, AsyncCommands, RedisResult};
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

        Ok(cached_value)
    }

    async fn get<T>(&self, key: &str) -> Result<T, CacheError>
    where
        T: DeserializeOwned,
    {
        let mut redis_connection_manager = self.redis_connection_manager.clone();
        let cached_value = redis_connection_manager
            .get(key)
            .await
            .map(|cached_str: String| serde_json::from_str::<T>(&cached_str))?
            .map_err(|err| CacheError::Unknown(err.to_string()))?;

        Ok(cached_value)
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

                redis_result
                    .map(|result| result.0)
                    .map_err(|err| CacheError::Unknown(err.to_string()))
            }
            Err(err) => {
                let msg = format!(
                    "Failed to serialize value for key '{key}' to set in Redis, err: '{err}'"
                );
                Err(CacheError::Unknown(msg))
            }
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

                redis_result
                    .map(|result| result.0)
                    .map_err(|err| CacheError::Unknown(err.to_string()))
            }
            Err(err) => {
                let msg = format!(
                    "Failed to serialize value for key '{key}' to set in Redis, err: '{err}'"
                );
                Err(CacheError::Unknown(msg))
            }
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

        redis_result
            .map(|result| result.0)
            .map_err(|err| CacheError::Unknown(err.to_string()))
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

        redis_result
            .map(|result| result.0)
            .map_err(|err| CacheError::Unknown(err.to_string()))
    }

    async fn wrap_fn<T, F, Fut>(&self, func: F, cache_key: &str) -> Result<T, CacheError>
    where
        T: Serialize + DeserializeOwned + Send + Sync,
        F: FnOnce() -> Fut + Send + Sync,
        Fut: Future<Output = Result<T, CacheError>> + Send + Sync,
    {
        let Ok(cached_value) = self.get(cache_key).await else {
            let func_value = func().await?;
            let set_result = self.set(cache_key, &func_value).await;

            if set_result.is_ok() {
                tracing::debug!("Cache for endpoint '{}' is set successfully", cache_key)
            } else {
                tracing::warn!(
                    "Cache for endpoint '{}' is failed to set with err: '{}'",
                    cache_key,
                    set_result.unwrap_err()
                )
            }

            return Ok(func_value);
        };

        Ok(cached_value)
    }
}
