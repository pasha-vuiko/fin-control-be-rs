use serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
pub struct AppConfig {
    // App
    pub port: u16,

    // Logging
    pub log_level: LogLevel,
    pub log_format: LogFormat,

    // Redis
    pub redis_sentinels: String,
    pub redis_name: String,
    pub redis_host: String,
    pub redis_port: u16,
    pub redis_ttl: usize, // seconds

    // Auth0
    pub auth_auth0_domain: String,

    // Database
    pub database_url: String,
}

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "lowercase")]
pub enum LogFormat {
    Json,
    Pretty,
}

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "lowercase")]
pub enum LogLevel {
    Error,
    Warn,
    Info,
    Debug,
    Trace,
    Silent,
}
