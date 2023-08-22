use crate::models::Id;
use crate::models::user::User;
use crate::models::session::{Session, NewSession};

use async_trait::async_trait;

#[async_trait]
pub trait SessionRepository {
    async fn get(&self, id: &Id<Session>) -> anyhow::Result<Option<Session>>;
    async fn find(&self, user_id: &Id<User>) -> anyhow::Result<Option<Vec<Session>>>;
    async fn insert(&self, source: NewSession) -> anyhow::Result<Session>;
    async fn delete(&self, id: &Id<Session>) -> anyhow::Result<Option<Session>>;
}