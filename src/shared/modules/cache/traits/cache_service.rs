use async_trait::async_trait;
use std::future::Future;

use crate::shared::modules::cache::errors::CacheError;

#[async_trait]
pub trait CacheService {
    async fn get_str(&self, key: &str) -> Result<String, CacheError>;

    async fn get<T>(&self, key: &str) -> Result<T, CacheError>
    where
        T: serde::de::DeserializeOwned;

    async fn set<T>(&self, key: &str, value: &T) -> Result<String, CacheError>
    where
        T: serde::Serialize + Send + Sync;

    async fn set_with_ttl<T>(&self, key: &str, value: &T, ttl: usize) -> Result<String, CacheError>
    where
        T: serde::Serialize + Send + Sync;

    async fn set_str(&self, key: &str, value: &str) -> Result<String, CacheError>;

    async fn set_str_with_ttl(
        &self,
        key: &str,
        value: &str,
        ttl: usize,
    ) -> Result<String, CacheError>;

    async fn wrap_fn<T, F, Fut>(&self, func: F, cache_key: &str) -> Result<T, CacheError>
    where
        T: serde::Serialize + serde::de::DeserializeOwned + Send + Sync,
        F: FnOnce() -> Fut + Send + Sync,
        Fut: Future<Output = Result<T, CacheError>> + Send + Sync;
}
