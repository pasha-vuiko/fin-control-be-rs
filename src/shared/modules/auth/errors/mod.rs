use onlyerror::Error;

#[derive(Error, Debug)]
pub enum AuthError {
    #[error("{0}")]
    NoAuthHeaderFound(String),
    #[error("{0}")]
    InvalidAuthHeader(String),
    #[error("{0}")]
    InvalidToken(String),
    #[error("{0}")]
    InvalidUserRoles(String),
}

impl From<alcoholic_jwt::ValidationError> for AuthError {
    fn from(value: alcoholic_jwt::ValidationError) -> Self {
        Self::InvalidToken(format!("JWT Validation Error: {}", value))
    }
}
