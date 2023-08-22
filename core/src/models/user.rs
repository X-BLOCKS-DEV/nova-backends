use chrono::DateTime;
use chrono::Utc;

use crate::models::Id;

#[derive(Debug)]
pub struct User {
    pub id: Id<User>,
    pub name: String,
    pub email: String,
    pub is_admin: bool,
    pub password_hash: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug)]
pub struct NewUser {
    pub id: Id<User>,
    pub name: String,
    pub email: String,
    pub is_admin: bool,
    pub password_hash: String,
}

impl NewUser {
    pub fn new(name: String, email: String, is_admin: bool, password_hash: String) -> Self {
        Self {
            id: Id::gen(),
            name,
            email,
            is_admin,
            password_hash,
        }
    }
}

#[derive(Debug)]
pub struct UpdateUser {
    pub id: Id<User>,
    pub name: Option<String>,
    pub email: Option<String>,
    pub is_admin: Option<bool>,
    pub password_hash: Option<String>,
}

impl UpdateUser {
    pub fn new(
        id: Id<User>,
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

#[derive(Debug)]
pub struct UpsertUser {
    pub id: Id<User>,
    pub name: String,
    pub email: String,
    pub is_admin: bool,
    pub password_hash: String,
}

impl UpsertUser {
    pub fn new(id: Id<User>, name: String, email: String, is_admin: bool, password_hash: String) -> Self {
        Self {
            id,
            name,
            email,
            is_admin,
            password_hash,
        }
    }
}