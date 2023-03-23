use alcoholic_jwt::{token_kid, validate, Validation, JWKS};

use crate::shared::errors::app_error::AppError;
use crate::shared::mods::auth::{claims::UserJwtClaims, roles::Roles};

#[derive(Clone)]
pub struct AuthService {
    jwks: JWKS,
    issuer: String,
}

impl AuthService {
    pub async fn from_auth_domain(jwks_domain: &str) -> Result<Self, AppError> {
        let issuer = format!("https://{}/", jwks_domain);
        let jwks_url = format!("{}{}", issuer, ".well-known/jwks.json");
        let jwks = Self::fetch_jwks(&jwks_url).await?;

        tracing::debug!("JWKS was successfully fetched");

        Ok(Self { jwks, issuer })
    }

    async fn fetch_jwks(uri: &str) -> Result<JWKS, Box<dyn std::error::Error>> {
        let res = reqwest::get(uri).await?;
        let val = res.json::<JWKS>().await?;

        Ok(val)
    }

    fn check_roles_match(required_roles: &[Roles], user_roles: &[Roles]) -> bool {
        for required_role in required_roles {
            if !user_roles.contains(required_role) {
                tracing::debug!(
                    "User does not have required role: {:?}, user roles: {:?}",
                    required_role,
                    user_roles
                );

                return false;
            }
        }
        true
    }

    pub async fn authenticate(
        &self,
        token: &str,
        required_roles: Vec<Roles>,
    ) -> Result<UserJwtClaims, AppError> {
        let claims = self.validate_token(token).await?;
        tracing::debug!("Token is validated successfully");

        match Self::check_roles_match(&required_roles, &claims.roles) {
            true => Ok(claims),
            false => Err(AppError::Forbidden {
                message: "User is not authorized to access this resource".to_string(),
            }),
        }
    }

    async fn validate_token(&self, token: &str) -> Result<UserJwtClaims, AppError> {
        let jwks = self.jwks.clone();
        let validations = vec![
            Validation::Issuer(self.issuer.to_string()),
            Validation::SubjectPresent,
            Validation::NotExpired,
        ];

        let token_kid_option = token_kid(token)?;

        match token_kid_option {
            Some(kid) => match jwks.find(&kid) {
                Some(jwk) => {
                    let valid_jwt = validate(token, jwk, validations)?;

                    let str_claims = valid_jwt.claims.to_string();
                    let claims = serde_json::from_str::<UserJwtClaims>(&str_claims)?;

                    Ok(claims)
                }
                None => {
                    let message = "Token is not valid, Specified key not found in JWKS set";
                    tracing::debug!("{}", message);

                    Err(AppError::Unauthorized {
                        message: message.to_string(),
                    })
                }
            },
            None => {
                let message = "Token is not valid, Key ID is not found in the token";
                tracing::debug!("{}", message);

                Err(AppError::Unauthorized {
                    message: message.to_string(),
                })
            }
        }
    }
}
