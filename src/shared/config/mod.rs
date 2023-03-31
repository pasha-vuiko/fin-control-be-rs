use serde::Deserialize;

pub mod tracing;

#[derive(Deserialize, Debug, Clone)]
pub struct AppConfig {
    // App
    pub port: u16,

    // Redis
    pub redis_config_sentinels: String,
    pub redis_config_name: String,
    pub redis_config_host: String,
    pub redis_config_port: u16,
    pub redis_ttl: usize, // seconds

    // Auth0
    pub auth_auth0_domain: String,

    // Database
    pub database_url: String,
}
