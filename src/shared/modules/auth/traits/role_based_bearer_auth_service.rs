use crate::shared::modules::auth::enums::roles::Roles;
use crate::shared::modules::auth::errors::AuthError;
use crate::shared::modules::auth::structs::user::User;

pub type DynamicAuthService = dyn AuthService + Send + Sync;

pub trait AuthService {
    fn authenticate(&self, token: &str, required_roles: Vec<Roles>) -> Result<User, AuthError>;
    fn get_user(&self, token: &str) -> Result<User, AuthError>;
}
