use crate::shared::mods::auth::claims::UserJwtClaims;
use crate::shared::mods::auth::roles::Roles;

pub struct User {
    pub id: String,
    pub name: String,
    pub first_name: String,
    pub last_name: String,
    pub nickname: String,
    pub email: String,
    pub email_verified: bool,
    pub roles: Vec<Roles>,
}

impl From<UserJwtClaims> for User {
    fn from(value: UserJwtClaims) -> Self {
        Self {
            id: value.sub,
            name: value.name,
            first_name: value.given_name,
            last_name: value.family_name,
            nickname: value.nickname,
            email: value.email,
            email_verified: value.email_verified,
            roles: value.roles,
        }
    }
}
