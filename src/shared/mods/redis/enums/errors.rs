use onlyerror::Error;

#[derive(Error, Debug)]
pub enum RedisServiceError {
    #[error("{0}")]
    Client(String),
    #[error("{0}")]
    ConnectionManager(String),
}
