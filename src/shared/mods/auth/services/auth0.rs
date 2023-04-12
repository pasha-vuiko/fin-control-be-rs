use alcoholic_jwt::{token_kid, validate, Validation, JWKS};
use base64;
use base64::Engine;
use serde::de::DeserializeOwned;

use crate::shared::errors::app_error::AppError;
use crate::shared::mods::auth::enums::errors::AuthError;
use crate::shared::mods::auth::enums::roles::Roles;
use crate::shared::mods::auth::structs::auth0_claims::Auth0JwtClaims;
use crate::shared::mods::auth::structs::user::User;
use crate::shared::mods::auth::traits::role_based_bearer_auth::AuthService;

#[derive(Clone)]
pub struct Auth0Service {
    jwks: JWKS,
    issuer: String,
}

impl Auth0Service {
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

    fn validate_token(&self, token: &str) -> Result<Auth0JwtClaims, AuthError> {
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
                    let claims =
                        serde_json::from_str::<Auth0JwtClaims>(&str_claims).map_err(|err| {
                            let msg = format!("Error while deserializing JWT claims: {}", err);
                            tracing::debug!(msg);

                            AuthError::InvalidToken(msg)
                        })?;

                    Ok(claims)
                }
                None => {
                    let message = "Token is not valid, Specified key not found in JWKS set";
                    tracing::debug!("{}", message);

                    Err(AuthError::InvalidToken(message.into()))
                }
            },
            None => {
                let message = "Token is not valid, Key ID is not found in the token";
                tracing::debug!("{}", message);

                Err(AuthError::InvalidToken(message.into()))
            }
        }
    }

    fn deserialize_jwt_part<T: DeserializeOwned>(part: &str) -> Result<T, AuthError> {
        let json = base64::engine::general_purpose::URL_SAFE_NO_PAD
            .decode(part.as_bytes())
            .map_err(|err| AuthError::InvalidToken(err.to_string()))?;

        serde_json::from_slice(&json).map_err(|err| AuthError::InvalidToken(err.to_string()))
    }
}

impl AuthService for Auth0Service {
    fn authenticate(&self, token: &str, required_roles: Vec<Roles>) -> Result<User, AuthError> {
        let claims = self.validate_token(token)?;
        tracing::debug!("Token is validated successfully");

        match Self::check_roles_match(&required_roles, &claims.roles) {
            true => Ok(claims.into()),
            false => Err(AuthError::InvalidUserRoles(
                "User is not authorized to access this resource".into(),
            )),
        }
    }

    fn get_user(&self, token: &str) -> Result<User, AuthError> {
        let parts = token.splitn(3, '.').collect::<Vec<&str>>();
        let claims_part = parts.get(1);

        match claims_part {
            Some(&claims_part) => {
                let jwt_claims: Auth0JwtClaims = Auth0Service::deserialize_jwt_part(claims_part)?;

                Ok(jwt_claims.into())
            }
            None => Err(AuthError::InvalidToken("invalid token".into())),
        }
    }
}