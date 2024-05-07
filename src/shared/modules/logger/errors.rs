use onlyerror::Error;
use tracing_subscriber::filter::{FromEnvError, ParseError};

#[derive(Debug, Error)]
pub enum CreateCrateLogFilterError {
    #[error("Failed to get env filter form env: {0}")]
    FromEnvError(FromEnvError),
    #[error("Failed to parse crates filters config: {0}")]
    ParseError(ParseError),
}

impl From<FromEnvError> for CreateCrateLogFilterError {
    fn from(error: FromEnvError) -> Self {
        Self::FromEnvError(error)
    }
}

impl From<ParseError> for CreateCrateLogFilterError {
    fn from(error: ParseError) -> Self {
        Self::ParseError(error)
    }
}