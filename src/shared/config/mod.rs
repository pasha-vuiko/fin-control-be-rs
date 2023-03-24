use serde::Deserialize;

pub mod tracing;

const DEFAULT_REDIS_TTL: usize = 0;
const DEFAULT_REDIS_HOST: &str = "localhost";
const DEFAULT_REDIS_PORT: u16 = 6379;

#[derive(Deserialize, Debug, Clone)]
pub struct AppConfig {
    // App
    pub port: u16,

    // Redis
    pub redis_config_sentinels: String,
    pub redis_config_name: String,
    #[serde(default = "default_redis_host")]
    pub redis_config_host: String,
    #[serde(default = "default_redis_port")]
    pub redis_config_port: u16,
    #[serde(default = "default_redis_ttl")]
    pub redis_ttl: usize, // seconds

    // Auth0
    pub auth_auth0_domain: String,
    pub auth_client_id: String,
    pub auth_client_secret: String,

    // Database
    pub database_url: String,
}

fn default_redis_ttl() -> usize {
    DEFAULT_REDIS_TTL
}

fn default_redis_host() -> String {
    DEFAULT_REDIS_HOST.into()
}

fn default_redis_port() -> u16 {
    DEFAULT_REDIS_PORT
}
