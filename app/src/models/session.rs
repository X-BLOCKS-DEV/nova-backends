use nova_core::models::session::{Session, NewSession};

use crate::models::DateTimeRfc3339;

pub struct SessionView {
    pub id: String,
    pub user_id: String,
    pub expired_at: DateTimeRfc3339,
    pub created_at: DateTimeRfc3339,
    pub updated_at: DateTimeRfc3339,
}

impl From<Session> for SessionView {
    fn from(s: Session) -> Self {
        Self {
            id: s.id.value.to_string(),
            user_id: s.user_id.value.to_string(),
            expired_at: s.expired_at.into(),
            created_at: s.created_at.into(),
            updated_at: s.updated_at.into(),
        }
    }
}

pub struct CreateSession {
    pub user_id: String,
    pub expired_at: DateTimeRfc3339,
}

impl CreateSession {
    pub fn new(user_id: String, expired_at: DateTimeRfc3339) -> Self {
        Self { user_id, expired_at }
    }
}

impl TryFrom<CreateSession> for NewSession {
    type Error = anyhow::Error;

    fn try_from(cs: CreateSession) -> Result<Self, Self::Error> {
        Ok(NewSession::new(cs.user_id.try_into()?))
    }
}