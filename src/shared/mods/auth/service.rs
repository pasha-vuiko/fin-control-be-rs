use alcoholic_jwt::{token_kid, validate, Validation, JWKS};
use base64;
use base64::Engine;
use serde::de::DeserializeOwned;

use crate::shared::errors::app_error::AppError;
use crate::shared::mods::auth::roles::Roles;
use crate::shared::mods::auth::structs::claims::UserJwtClaims;
use crate::shared::mods::auth::structs::user::User;

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

    pub fn check_user_roles(required_roles: &[Roles], user: &User) -> Result<bool, AppError> {
        let user_roles = user.roles.clone();
        let roles_match = Self::check_roles_match(required_roles, &user_roles);

        if roles_match {
            Ok(true)
        } else {
            Err(AppError::Forbidden {
                message: "User is not authorized to access this resource".into(),
            })
        }
    }

    pub fn authenticate(
        &self,
        token: &str,
        required_roles: Vec<Roles>,
    ) -> Result<UserJwtClaims, AppError> {
        let claims = self.validate_token(token)?;
        tracing::debug!("Token is validated successfully");

        match Self::check_roles_match(&required_roles, &claims.roles) {
            true => Ok(claims),
            false => Err(AppError::Forbidden {
                message: "User is not authorized to access this resource".into(),
            }),
        }
    }

    async fn fetch_jwks(uri: &str) -> Result<JWKS, Box<dyn std::error::Error>> {
        let res = reqwest::get(uri).await?;
        let val = res.json::<JWKS>().await?;

        Ok(val)
    }

    fn check_roles_match(required_roles: &[Roles], user_roles: &[Roles]) -> bool {
        if required_roles.is_empty() {
            return true;
        }

        for required_role in required_roles {
            if user_roles.contains(required_role) {
                return true;
            }
        }

        tracing::debug!(
            "User does not have one of required roles: {:?}, user roles: {:?}",
            required_roles,
            user_roles
        );

        false
    }

    fn validate_token(&self, token: &str) -> Result<UserJwtClaims, AppError> {
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
                        message: message.into(),
                    })
                }
            },
            None => {
                let message = "Token is not valid, Key ID is not found in the token";
                tracing::debug!("{}", message);

                Err(AppError::Unauthorized {
                    message: message.into(),
                })
            }
        }
    }

    pub fn get_claims(&self, token: &str) -> Result<UserJwtClaims, AppError> {
        let parts = token.splitn(3, '.').collect::<Vec<&str>>();
        let claims_part = parts.get(1);

        match claims_part {
            Some(&claims_part) => {
                AuthService::deserialize_jwt_part(claims_part).map_err(|err| err.into())
            }
            None => Err(AppError::Internal {
                message: "Token is not valid".into(),
            }),
        }
    }

    fn deserialize_jwt_part<T: DeserializeOwned>(
        part: &str,
    ) -> Result<T, Box<dyn std::error::Error>> {
        let json = base64::engine::general_purpose::URL_SAFE_NO_PAD.decode(part.as_bytes())?;

        serde_json::from_slice(&json).map_err(Into::into)
    }
}
