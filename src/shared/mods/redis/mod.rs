use crate::shared::config::AppConfig;
use crate::shared::mods::redis::redis_service::RedisService;

pub mod redis_service;

pub async fn get_redis_service(config: &AppConfig) -> RedisService {
    let redis_uri = format!(
        "redis://{}:{}",
        config.redis_config_host, config.redis_config_port
    );

    // TODO Add retry strategy
    let redis_client = redis::Client::open(redis_uri).expect("Can't create Redis client");
    let redis_connection_manager = redis_client
        .get_tokio_connection_manager()
        .await
        .expect("Can't create Redis connection manager");

    RedisService::new(redis_connection_manager, config.redis_ttl)
}
