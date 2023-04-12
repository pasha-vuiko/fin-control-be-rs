use crate::shared::mods::auth::enums::roles::Roles;
use crate::shared::mods::auth::structs::auth0_claims::Auth0JwtClaims;

#[derive(Debug, Clone)]
pub struct User {
    pub id: String,
    pub name: String,
    pub nickname: String,
    pub email: String,
    pub email_verified: bool,
    pub roles: Vec<Roles>,
}
impl User {
    pub fn is_admin(&self) -> bool {
        self.roles.contains(&Roles::Admin)
    }
}

impl From<Auth0JwtClaims> for User {
    fn from(value: Auth0JwtClaims) -> Self {
        Self {
            id: value.sub,
            name: value.name,
            nickname: value.nickname,
            email: value.email,
            email_verified: value.email_verified,
            roles: value.roles,
        }
    }
}
