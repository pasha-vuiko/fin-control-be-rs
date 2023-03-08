use clap::Parser;

pub mod tracing;

#[derive(Parser)]
pub struct Config {
    // App
    #[clap(long, env)]
    pub port: u16,

    // Redis
    #[clap(long, env)]
    pub redis_config_sentinels: String,
    #[clap(long, env)]
    pub redis_config_name: String,
    #[clap(long, env)]
    pub redis_config_host: String,
    #[clap(long, env)]
    pub redis_config_port: u16,
    #[clap(long, env)]
    pub redis_ttl: u16, // seconds

    // Auth0
    #[clap(long, env)]
    pub auth_auth0_domain: String,
    #[clap(long, env)]
    pub auth_client_id: String,
    #[clap(long, env)]
    pub auth_client_secret: String,

    // Database
    #[clap(long, env)]
    pub database_url: String,
}
