use std::sync::Arc;

use crate::models::user::{ CreateUser, UpdateUserView, UserView, UpsertUserView };
use nova_adapter::modules::RepositoriesModuleExt;
use nova_core::repositories::user::UserRepository;
use nova_core::models::user::{ UpdateUser, UpsertUser };

pub struct UserUseCase<R: RepositoriesModuleExt> {
    repositories: Arc<R>,
}

impl<R: RepositoriesModuleExt> UserUseCase<R> {
    pub fn new(repositories: Arc<R>) -> Self {
        Self { repositories }
    }

    pub async fn get_user(&self, id: String) -> anyhow::Result<Option<UserView>> {
        let resp = self.repositories.user_repository().get(&id.try_into()?).await?;

        match resp {
            Some(u) => Ok(Some(u.into())),
            None => Ok(None),
        }
    }

    pub async fn register_user(&self, source: CreateUser) -> anyhow::Result<UserView> {
        let user_view = self.repositories.user_repository().insert(source.try_into()?).await?;

        Ok(user_view.into())
    }

    pub async fn update_user(&self, source: UpdateUserView) -> anyhow::Result<UserView> {
        let update_user = UpdateUser::new(
            source.id.try_into()?,
            source.name,
            source.email,
            source.is_admin,
            source.password_hash
        );

        let user_view = self.repositories.user_repository().update(update_user).await?;

        Ok(user_view.into())
    }

    pub async fn upsert_user(&self, source: UpsertUserView) -> anyhow::Result<UserView> {
        let upsert_user = UpsertUser::new(
            source.id.try_into()?,
            source.name,
            source.email,
            source.is_admin,
            source.password_hash
        );

        let user_view = self.repositories.user_repository().upsert(upsert_user).await?;

        Ok(user_view.into())
    }

    pub async fn delete_user(&self, id: String) -> anyhow::Result<Option<UserView>> {
        let resp = self.repositories.user_repository().delete(&id.try_into()?).await?;

        match resp {
            Some(u) => Ok(Some(u.into())),
            None => Ok(None),
        }
    }
}
