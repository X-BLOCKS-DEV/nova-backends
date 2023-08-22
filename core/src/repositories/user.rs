use crate::models::Id;
use crate::models::user::{User, NewUser, UpdateUser, UpsertUser};
use async_trait::async_trait;

#[async_trait]
pub trait UserRepository {
    async fn get(&self, id: &Id<User>) -> anyhow::Result<Option<User>>;
    // async fn find(&self, status: Option<UserSearchQuery>) -> anyhow::Result<Option<Vec<User>>>;
    async fn insert(&self, source: NewUser) -> anyhow::Result<User>;
    async fn update(&self, source: UpdateUser) -> anyhow::Result<User>;
    async fn upsert(&self, source: UpsertUser) -> anyhow::Result<User>;
    async fn delete(&self, id: &Id<User>) -> anyhow::Result<Option<User>>;
}