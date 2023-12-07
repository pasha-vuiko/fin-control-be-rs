use async_trait::async_trait;
use redis::{aio::ConnectionManager, AsyncCommands, RedisResult};
use serde::de::DeserializeOwned;
use serde::Serialize;
use std::future::Future;

use crate::shared::modules::cache::errors::CacheError;
use crate::shared::modules::cache::traits::cache_service::CacheService;

#[derive(Clone)]
pub struct RedisService {
    connection_manager: ConnectionManager,
    default_ttl: usize,
}

impl RedisService {
    pub fn new(redis_connection_manager: ConnectionManager, default_ttl: usize) -> Self {
        Self {
            connection_manager: redis_connection_manager,
            default_ttl,
        }
    }
}

#[async_trait]
impl CacheService for RedisService {
    async fn get_str(&self, key: &str) -> Result<String, CacheError> {
        let cached_value = self.connection_manager.clone().get(key).await?;

        Ok(cached_value)
    }

    async fn get<T>(&self, key: &str) -> Result<T, CacheError>
    where
        T: DeserializeOwned,
    {
        let cached_value = self
            .connection_manager
            .clone()
            .get::<_, Option<String>>(key)
            .await
            .map_err(|err| CacheError::Unknown(err.to_string()))?
            .ok_or(CacheError::KeyNotFound(key.to_string()))
            .map(|cached_str| serde_json::from_str(&cached_str))?
            .map_err(|err| CacheError::FailedToParseResponse(err.to_string()))?;

        Ok(cached_value)
    }

    async fn set<T>(&self, key: &str, value: &T) -> Result<String, CacheError>
    where
        T: Serialize + Send + Sync,
    {
        // TODO Find out if 0 TTL is valid value
        self.set_with_ttl(key, value, self.default_ttl).await
    }

    async fn set_with_ttl<T>(&self, key: &str, value: &T, ttl: usize) -> Result<String, CacheError>
    where
        T: Serialize + Send + Sync,
    {
        let serialized_value = serde_json::to_string(value).map_err(|err| {
            CacheError::Unknown(format!(
                "Failed to serialize value for key '{key}' to set in Redis, err: '{err}'"
            ))
        })?;

        let mut connection_manager = self.connection_manager.clone();
        let redis_result: RedisResult<(String, i32)> = redis::pipe()
            .atomic()
            .set(key, &serialized_value)
            .expire(key, ttl as i64)
            .query_async(&mut connection_manager)
            .await;

        redis_result
            .map(|result| result.0)
            .map_err(|err| CacheError::Unknown(err.to_string()))
    }

    async fn set_str(&self, key: &str, value: &str) -> Result<String, CacheError> {
        // TODO Find out if 0 TTL is valid value
        self.set_str_with_ttl(key, value, self.default_ttl).await
    }

    async fn set_str_with_ttl(
        &self,
        key: &str,
        value: &str,
        ttl: usize,
    ) -> Result<String, CacheError> {
        let mut connection_manager = self.connection_manager.clone();
        let redis_result: RedisResult<(String, i32)> = redis::pipe()
            .atomic()
            .set(key, value)
            .expire(key, ttl as i64)
            .query_async(&mut connection_manager)
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
