use chrono::DateTime;
use chrono::Utc;

use crate::models::user::User;
use crate::models::Id;

#[derive(Debug)]
pub struct Session {
    pub id: Id<Session>,
    pub user_id: Id<User>,
    pub expired_at: DateTime<Utc>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug)]
pub struct NewSession {
    pub id: Id<Session>,
    pub user_id: Id<User>,
}

impl NewSession {
    pub fn new(user_id: Id<User>) -> Self {
        Self {
            id: Id::gen(),
            user_id,
        }
    }
}

