use chrono::{DateTime, Utc};
use sqlx::FromRow;

use nova_core::models::user::{User, NewUser, UpdateUser, UpsertUser};

#[derive(FromRow, Debug)]
pub struct StoredUser {
    pub id: String,
    pub name: String,
    pub email: String,
    pub is_admin: bool,
    pub password_hash: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl TryFrom<StoredUser> for User {
    type Error = anyhow::Error;

    fn try_from(su: StoredUser) -> Result<Self, Self::Error> {
        Ok(User {
            id: su.id.try_into()?,
            name: su.name,
            email: su.email,
            is_admin: su.is_admin,
            password_hash: su.password_hash,
            created_at: su.created_at,
            updated_at: su.updated_at,
        })
    }
}

#[derive(FromRow, Debug)]
pub struct InsertStoredUser {
    pub id: String,
    pub name: String,
    pub email: String,
    pub is_admin: bool,
    pub password_hash: String,
}

impl From<NewUser> for InsertStoredUser {
    fn from(nu: NewUser) -> Self {
        InsertStoredUser {
            id: nu.id.value.to_string(),
            name: nu.name,
            email: nu.email,
            is_admin: nu.is_admin,
            password_hash: nu.password_hash
        }
    }
}

#[derive(FromRow, Debug)]
pub struct UpdateStoredUser {
    pub id: String,
    pub name: Option<String>,
    pub email: Option<String>,
    pub is_admin: Option<bool>,
    pub password_hash: Option<String>,
}

impl From<UpdateUser> for UpdateStoredUser {
    fn from(uu: UpdateUser) -> Self {
        UpdateStoredUser {
            id: uu.id.value.to_string(),
            name: uu.name,
            email: uu.email,
            is_admin: uu.is_admin,
            password_hash: uu.password_hash
        }
    }
}

#[derive(FromRow, Debug)]
pub struct UpsertStoredUser {
    pub id: String,
    pub name: String,
    pub email: String,
    pub is_admin: bool,
    pub password_hash: String,
}

impl From<UpsertUser> for UpsertStoredUser {
    fn from(uu: UpsertUser) -> Self {
        UpsertStoredUser {
            id: uu.id.value.to_string(),
            name: uu.name,
            email: uu.email,
            is_admin: uu.is_admin,
            password_hash: uu.password_hash
        }
    }
}