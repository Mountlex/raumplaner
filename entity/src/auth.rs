use axum_login::{AuthUser, secrecy::SecretVec};
use crate::entities::user;

impl AuthUser for user::Model {
    fn get_id(&self) -> String {
        format!("{}", self.id)
    }

    fn get_password_hash(&self) -> SecretVec<u8> {
        SecretVec::new(self.password_hash.clone().into())
    }
}