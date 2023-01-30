use axum::async_trait;
use axum_login::{secrecy::SecretVec, AuthUser, RequireAuthorizationLayer, UserStore};
use entity::sea_orm_active_enums::RoleType;
use sea_orm::{DatabaseConnection, EntityTrait};

use entity::user;

type Result<T = ()> = std::result::Result<T, eyre::Report>;

pub type AuthContext = axum_login::extractors::AuthContext<User, SeaUserStore, Role>;

pub type RequireAuth = RequireAuthorizationLayer<User, Role>;

#[derive(Debug, Clone)]
pub struct User {
    pub id: i32,
    pub password: String,
    pub role: Role,
    pub name: String,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Role {
    Admin,
    DefaultUser,
}

impl From<RoleType> for Role {
    fn from(value: RoleType) -> Self {
        match value {
            RoleType::Admin => Role::Admin,
            RoleType::DefaultUser => Role::DefaultUser,
        }
    }
}

impl AuthUser<Role> for User {
    fn get_id(&self) -> String {
        format!("{}", self.id)
    }

    fn get_password_hash(&self) -> axum_login::secrecy::SecretVec<u8> {
        SecretVec::new(self.password.clone().into())
    }
}

#[derive(Debug, Clone)]
pub struct SeaUserStore {
    conn: DatabaseConnection,
}

impl SeaUserStore {
    pub fn new(conn: &DatabaseConnection) -> Self {
        Self { conn: conn.clone() }
    }
}

#[async_trait]
impl UserStore<Role> for SeaUserStore {
    type User = User;

    async fn load_user(&self, user_id: &str) -> Result<Option<User>> {
        let user = user::Entity::find_by_id(user_id.parse::<i32>().unwrap())
            .one(&self.conn)
            .await?;

        match user {
            Some(u) => Ok(Some(User {
                id: u.id,
                password: u.password_hash,
                role: u.role.into(),
                name: u.name,
            })),
            None => Ok(None),
        }
    }
}
