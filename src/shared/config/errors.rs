use onlyerror::Error;

#[derive(Debug, Clone, Error)]
pub enum ConfigErrors {
    #[error("{0}")]
    ConfigFileError(String),
    #[error("{0}")]
    ParseError(String),
}
