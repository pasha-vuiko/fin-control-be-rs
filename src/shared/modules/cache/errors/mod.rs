use onlyerror::Error;
use redis::RedisError;

#[derive(Error, Debug)]
pub enum CacheError {
    #[error("{0}")]
    Unknown(String),
    #[error("{0}")]
    KeyNotFound(String),
    #[error("{0}")]
    FailedToParseResponse(String),
}

impl From<RedisError> for CacheError {
    fn from(source: RedisError) -> Self {
        Self::Unknown(source.to_string())
    }
}
