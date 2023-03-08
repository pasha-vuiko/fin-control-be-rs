use std::env;

pub mod redis_service;
use crate::shared::mods::redis::redis_service::RedisService;

pub async fn get_redis_service() -> RedisService {
    let redis_host =
        env::var("REDIS_CONFIG_HOST").expect("Env var 'REDIS_CONFIG_HOST' is not specified");
    let redis_port =
        env::var("REDIS_CONFIG_PORT").expect("Env var 'REDIS_CONFIG_PORT' is not specified");
    let redis_uri = format!("redis://{}:{}", redis_host, redis_port);

    let redis_client = redis::Client::open(redis_uri).expect("Can't create Redis client");
    let redis_connection_manager = redis_client
        .get_tokio_connection_manager()
        .await
        .expect("Can't create Redis connection manager");

    RedisService::new(redis_connection_manager)
}
