use alcoholic_jwt::{token_kid, validate, Validation, JWKS};
use base64;
use base64::Engine;
use serde::de::DeserializeOwned;

use crate::shared::errors::http_error::HttpError;
use crate::shared::modules::auth::enums::roles::Roles;
use crate::shared::modules::auth::errors::AuthError;
use crate::shared::modules::auth::structs::auth0_claims::Auth0JwtClaims;
use crate::shared::modules::auth::structs::user::User;
use crate::shared::modules::auth::traits::role_based_bearer_auth_service::AuthService;

#[derive(Clone)]
pub struct Auth0Service {
    jwks: JWKS,
    issuer: String,
}

impl Auth0Service {
    pub async fn from_auth_domain(jwks_domain: &str) -> Result<Self, HttpError> {
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
        let validations = vec![
            Validation::Issuer(self.issuer.to_string()),
            Validation::SubjectPresent,
            Validation::NotExpired,
        ];

        token_kid(token)?
            .ok_or_else(|| {
                let message = "Token is not valid, Key ID is not found in the token";
                tracing::debug!("{}", message);

                AuthError::InvalidToken(message.into())
            })
            .and_then(|kid| self.get_jwk_by_kid(&kid))
            .and_then(|jwk| {
                let valid_jwt = validate(token, jwk, validations)?;

                let str_claims = valid_jwt.claims.to_string();
                let claims = Self::str_claims_to_claims(&str_claims)?;

                Ok(claims)
            })
    }

    fn deserialize_jwt_part<T: DeserializeOwned>(part: &str) -> Result<T, AuthError> {
        let json = base64::engine::general_purpose::URL_SAFE_NO_PAD
            .decode(part.as_bytes())
            .map_err(|err| AuthError::InvalidToken(err.to_string()))?;

        serde_json::from_slice(&json).map_err(|err| AuthError::InvalidToken(err.to_string()))
    }

    fn str_claims_to_claims(str_claims: &str) -> Result<Auth0JwtClaims, AuthError> {
        serde_json::from_str::<Auth0JwtClaims>(str_claims).map_err(|err| {
            let msg = format!("Error while deserializing JWT claims: {}", err);
            tracing::debug!(msg);

            AuthError::InvalidToken(msg)
        })
    }

    fn get_jwk_by_kid(&self, kid: &str) -> Result<&alcoholic_jwt::JWK, AuthError> {
        self.jwks.find(kid).ok_or_else(|| {
            let message = "Token is not valid, Specified key not found in JWKS set";
            tracing::debug!("{}", message);

            AuthError::InvalidToken(message.into())
        })
    }
}

impl AuthService for Auth0Service {
    fn authenticate(&self, token: &str, required_roles: Vec<Roles>) -> Result<User, AuthError> {
        let claims = self.validate_token(token)?;
        tracing::debug!("Token is validated successfully");

        let roles_match = Self::check_roles_match(&required_roles, &claims.roles);

        if roles_match {
            Ok(claims.into())
        } else {
            Err(AuthError::InvalidUserRoles(
                "User is not authorized to access this resource".into(),
            ))
        }
    }

    fn get_user(&self, token: &str) -> Result<User, AuthError> {
        let parts = token.splitn(3, '.').collect::<Vec<&str>>();
        let claims_part = parts.get(1);

        let Some(&claims_part) = claims_part else {
            return Err(AuthError::InvalidToken("invalid token".into()));
        };

        let jwt_claims: Auth0JwtClaims = Auth0Service::deserialize_jwt_part(claims_part)?;

        Ok(jwt_claims.into())
    }
}
