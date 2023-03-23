use crate::shared::mods::auth::roles::Roles;
use chrono::FixedOffset;
use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct UserJwtClaims {
    pub given_name: String,
    pub family_name: String,
    pub nickname: String,
    pub name: String,
    pub picture: String,
    pub locale: String,
    pub updated_at: chrono::DateTime<FixedOffset>,
    pub email: String,
    pub email_verified: bool,
    pub iat: u32,
    pub exp: u32,
    pub nonce: String,
    pub iss: String,
    pub aud: String,
    pub sub: String,
    #[serde(rename(deserialize = "https://meta.com/roles"))]
    pub roles: Vec<Roles>,
}
