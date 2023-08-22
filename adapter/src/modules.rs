use nova_core::models::user::User;
use nova_core::repositories::user::UserRepository;

use crate::persistence::postgres::Db;
use crate::repositories::DatabaseRepositoryImpl;

pub struct RepositoriesModule {
    user_repository: DatabaseRepositoryImpl<User>,
}

pub trait RepositoriesModuleExt {
    type UserRepo: UserRepository;

    fn user_repository(&self) -> &Self::UserRepo;
}

impl RepositoriesModuleExt for RepositoriesModule {
    type UserRepo = DatabaseRepositoryImpl<User>;

    fn user_repository(&self) -> &Self::UserRepo {
        &self.user_repository
    }
}

impl RepositoriesModule {
    pub fn new(db: Db) -> Self {
        let user_repository = DatabaseRepositoryImpl::new(db.clone());
        Self {
            user_repository,
        }
    }
}