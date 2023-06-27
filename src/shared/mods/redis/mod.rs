use crate::shared::mods::redis::enums::errors::RedisServiceError;
use crate::shared::mods::redis::redis_service::RedisService;

pub mod enums;
pub mod redis_service;

pub struct RedisServiceBuilder {
    host: String,
    port: u16,
    default_ttl: Option<usize>,
}

impl RedisServiceBuilder {
    pub fn new(host: &str, port: u16) -> Self {
        Self {
            host: host.to_string(),
            port,
            default_ttl: None,
        }
    }

    pub fn with_default_ttl(mut self, default_ttl: usize) -> Self {
        self.default_ttl = Some(default_ttl);

        self
    }

    pub async fn build(self) -> Result<RedisService, RedisServiceError> {
        let redis_uri = format!("redis://{}:{}", self.host, self.port);
        
        let redis_client = redis::Client::open(redis_uri)
            .map_err(|err| RedisServiceError::Client(err.to_string()))?;
        let redis_connection_manager = redis_client
            .get_tokio_connection_manager()
            .await
            .map_err(|err| RedisServiceError::ConnectionManager(err.to_string()))?;

        let redis_service =
            RedisService::new(redis_connection_manager, self.default_ttl.unwrap_or(0));

        Ok(redis_service)
    }
}
