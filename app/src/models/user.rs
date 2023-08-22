use crate::models::DateTimeRfc3339;
use nova_core::models::user::{ User, NewUser };

pub struct UserView {
    pub id: String,
    pub name: String,
    pub email: String,
    pub is_admin: bool,
    pub password_hash: String,
    pub created_at: DateTimeRfc3339,
    pub updated_at: DateTimeRfc3339,
}

impl From<User> for UserView {
    fn from(u: User) -> Self {
        Self {
            id: u.id.value.to_string(),
            name: u.name,
            email: u.email,
            is_admin: u.is_admin,
            password_hash: u.password_hash,
            created_at: u.created_at.into(),
            updated_at: u.updated_at.into(),
        }
    }
}

pub struct CreateUser {
    pub name: String,
    pub email: String,
    pub is_admin: bool,
    pub password_hash: String,
}

impl CreateUser {
    pub fn new(name: String, email: String, is_admin: bool, password_hash: String) -> Self {
        Self { name, email, is_admin, password_hash }
    }
}

impl TryFrom<CreateUser> for NewUser {
    type Error = anyhow::Error;

    fn try_from(cu: CreateUser) -> Result<Self, Self::Error> {
        Ok(NewUser::new(cu.name, cu.email, cu.is_admin, cu.password_hash))
    }
}

pub struct UpdateUserView {
    pub id: String,
    pub name: Option<String>,
    pub email: Option<String>,
    pub is_admin: Option<bool>,
    pub password_hash: Option<String>,
}

impl UpdateUserView {
    pub fn new(
        id: String,
        name: Option<String>,
        email: Option<String>,
        is_admin: Option<bool>,
        password_hash: Option<String>
    ) -> Self {
        Self {
            id,
            name,
            email,
            is_admin,
            password_hash,
        }
    }
}

pub struct UpsertUserView {
    pub id: String,
    pub name: String,
    pub email: String,
    pub is_admin: bool,
    pub password_hash: String,
}

impl UpsertUserView {
    pub fn new(
        id: String,
        name: String,
        email: String,
        is_admin: bool,
        password_hash: String
    ) -> Self {
        Self {
            id,
            name,
            email,
            is_admin,
            password_hash,
        }
    }
}
